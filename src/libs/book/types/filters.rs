use std::{ cmp::Eq, collections::HashSet, fmt, hash::Hash };

use crate::{
   csv_utils::{ CsvWriter, CsvHeader, enumerate_csv },
   string_utils::s
};

/// types that are filterable: returns the value used as the filter
pub trait Filter<T> { fn filter(&self) -> T; }

/// types that filter
pub trait Container<T> {
   fn contains<F: Filter<T>>(&self, target: &F) -> bool;
}

/// A sieve: allows all values through the filter
pub struct Sieve;

impl<T> Container<T> for Sieve {
   fn contains<F: Filter<T>>(&self, _t: &F) -> bool { true }
}

pub trait PermissionList<T> : CsvWriter + CsvHeader {
   fn set(&self) -> Vec<T>;
}

fn default_ncols() -> usize { 1 }

struct Stringy { s: String }
impl CsvWriter for Stringy {
   fn ncols(&self) -> usize { 1 }
   fn as_csv(&self) -> String { self.s.clone() }
}
fn mk_stringy<U: fmt::Display>(s: U) -> Stringy {
   Stringy { s: format!("{s}") }
}

fn default_as_csv<T: fmt::Display, P: PermissionList<T>>(p: &P) -> String {
   enumerate_csv(&p.set().into_iter().map(mk_stringy).collect())
}
fn default_header() -> String { s("member") }

/// A Whitelist allows only the values so contained
pub struct WhiteList<T:Eq + Hash> { set: HashSet<T> }

pub fn mk_whitelist<T:Eq + Hash>(v: Vec<T>) -> WhiteList<T> {
   WhiteList { set: v.into_iter().collect() }
}

impl<T:Eq + Hash> Container<T> for WhiteList<T> {
   fn contains<F: Filter<T>>(&self, t: &F) -> bool {
      self.set.contains(&t.filter())
   }
}

impl<T:Eq + Hash + Clone + fmt::Display> CsvWriter for WhiteList<T> {
   fn ncols(&self) -> usize { default_ncols() }
   fn as_csv(&self) -> String { default_as_csv(self) }
}
impl<T:Eq + Hash> CsvHeader for WhiteList<T> { 
   fn header(&self) -> String { default_header() }
}

impl<T:Clone + Eq + Hash + fmt::Display> PermissionList<T> for WhiteList<T> {
   fn set(&self) -> Vec<T> { self.set.clone().into_iter().collect() }
}

/// A Blacklist forbids all values on its list
pub struct BlackList<T:Eq + Hash> { set: HashSet<T> }

pub fn mk_blacklist<T:Eq + Hash>(v: Vec<T>) -> BlackList<T> {
   BlackList { set: v.into_iter().collect() }
}

impl<T:Eq + Hash> Container<T> for BlackList<T> {
   fn contains<F: Filter<T>>(&self, t: &F) -> bool {
      !self.set.contains(&t.filter())
   }
}

impl<T:Clone + Eq + Hash + fmt::Display> PermissionList<T> for BlackList<T> {
   fn set(&self) -> Vec<T> { self.set.clone().into_iter().collect() }
}
impl<T:Eq + Hash + Clone + fmt::Display> CsvWriter for BlackList<T> {
   fn ncols(&self) -> usize { default_ncols() }
   fn as_csv(&self) -> String { default_as_csv(self) }
}
impl<T:Eq + Hash> CsvHeader for BlackList<T> {
   fn header(&self) -> String { default_header() }
}

// ----- TESTS -------------------------------------------------------

#[cfg(test)]
#[cfg(not(tarpaulin_include))]
mod functional_tests {
   use super::*;
   use crate::{
      create_testing,
      err_utils::ErrStr,
      string_utils::words
   };
   use paste::paste;

   create_testing!("types::filters");
   run!("mk_whitelist", {
      let wl = mk_whitelist(words("red green blue"));
      println!("A whitelist is\n{}", wl.as_csv());
   });
}

#[cfg(test)]
#[cfg(not(tarpaulin_include))]
mod tests {
   use super::*;
   use crate::string_utils::words;

   impl Filter<String> for String {
      fn filter(&self) -> String { self.clone() }
   }

   fn lorem() -> Vec<String> {
      words("The quick, brown fox jumps over the lazy dog.")
   }
   const LEN: usize = 9;

   fn the_the() -> Vec<String> {
      words("The the")
   }

   #[test] fn test_sieve() {
      let sieve = Sieve;
      let ans: Vec<String> =
         lorem().into_iter().filter(|s| sieve.contains(s)).collect();
      assert_eq!(LEN, ans.len());
   }

   #[test] fn test_whitelist() {
      let wl = mk_whitelist(the_the());
      let mut ans = lorem();
      ans.retain(|s| wl.contains(s));
      assert_eq!(2, ans.len());
   }

   #[test] fn test_blacklist() {
      let wl = mk_blacklist(the_the());
      let mut ans = lorem();
      ans.retain(|s| wl.contains(s));
      assert_eq!(LEN - 2, ans.len());
   }
}
