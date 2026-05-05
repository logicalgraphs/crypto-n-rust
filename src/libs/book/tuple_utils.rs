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

#[cfg(test)]
#[cfg(not(tarpaulin_include))]
pub mod functional_tests {
   use super::*;
   use paste::paste;
   use crate::{ create_testing, err_utils::ErrStr };

   fn plus1() -> impl Fn((i32, i32)) -> (i32, i32) { first(|a| a + 1) }
   fn upper<A>() -> impl Fn((A, &'static str)) -> (A, String) {
      second(|a: &str| a.to_uppercase())
   }

   create_testing!("tuple_utils");

   run_with!("fst", (1, "two"), fst);
   run_with!("snd", (1, "two"), snd);
   run_with!("swap", (1, "two"), swap);
   run_with!("first_plus_1", (5, 7), plus1());
   run_with!("second_uppercase", (6, "seven"), upper());

   run_all_functional_tests!();

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

