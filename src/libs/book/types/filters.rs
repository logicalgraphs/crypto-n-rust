use std::{ cmp::Eq, collections::HashSet, hash::Hash };

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

// ----- TESTS -------------------------------------------------------

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
