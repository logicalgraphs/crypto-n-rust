pub fn fst<T: Clone, U>(p: (T, U)) -> T { p.0.clone() }
pub fn snd<T, U: Clone>(p: (T, U)) -> U { p.1.clone() }

pub fn swap<T, U>(t: (T, U)) -> (U, T) {
   (t.1, t.0)
}

// Now we must consider if tuple_utils is a trojan horse for arrow functions

// Put another way: do I now need arrow_utils?

pub fn first<A, B, C>(f: impl Fn(A) -> C) -> impl Fn((A, B)) -> (C, B) {
   move |(a, b)| (f(a), b)
}

pub fn second<A, B, C>(f: impl Fn(B) -> C) -> impl Fn((A, B)) -> (A, C) {
   move |(a, b)| (a, f(b))
}

/// a Partition is a particular tuple: a pair of sets of the same type,
/// discriminated by something

pub type Partition<T> = (Vec<T>, Vec<T>);

// ----- TESTS -------------------------------------------------------
   
#[cfg(not(tarpaulin_include))]
pub mod functional_tests {
   use std::fmt::Debug;
   use super::*;
   use crate::err_utils::ErrStr;

   fn report<A: Debug + Clone, B: Debug + Clone, RES: Debug>(test: &str,
         t: (A, B), f: impl Fn((A, B)) -> RES) -> ErrStr<usize> {
      println!("\ntuple_utils::run_{test} functional test

	tuple: {:?}, function: {test}, result: {:?}

tuple_utils::{test}:...ok", t.clone(), f(t));
      Ok(1)
   }

   fn run_fst() -> ErrStr<usize> { report("fst", (1, "two"), fst) }
   fn run_snd() -> ErrStr<usize> { report("snd", (1, "two"), snd) }
   fn run_swap() -> ErrStr<usize> { report("swap", (1, "two"), swap) }
   fn run_first() -> ErrStr<usize> {
      let plus1 = first(|a| a + 1);
      report("first+1", (5, "seven"), plus1)
   }
   fn upper<A>() -> impl Fn((A, &'static str)) -> (A, String) {
      second(|a: &str| a.to_uppercase())
   }

   fn run_second() -> ErrStr<usize> {
      report("second_uppercase", (6, "seven"), upper())
   }

   pub fn runoff() -> ErrStr<usize> {
      println!("\ntuple_utils functional tests\n");
      let a = run_fst()?;
      let b = run_snd()?;
      let c = run_swap()?;
      let d = run_first()?;
      let e = run_second()?;
      Ok(a+b+c+d+e)
   }

#[cfg(test)]
mod tests {
   use super::*;
   use crate::string_utils::s;

   #[test] fn test_fst() { assert_eq!(1, fst((1, "two"))); }
   #[test] fn test_snd() { assert_eq!("two", snd((1, "two"))); }
   #[test] fn test_swap() { assert_eq!(("two", 1), swap((1, "two"))); }
   #[test] fn test_first_plus_1() {
      assert_eq!((6, "seven"), first(|a| a+1)((5, "seven")));
   }
   #[test] fn test_second_uppercase() {
      assert_eq!((6, s("SEVEN")), upper()((6, "seven")));
   }
}
}

