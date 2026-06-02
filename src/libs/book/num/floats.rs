use std::hash::{Hash,Hasher};

#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SafeFloat(f32);

impl Eq for SafeFloat {}
impl Hash for SafeFloat {
   fn hash<H: Hasher>(&self, state: &mut H) {
      let bits = if self.0.is_nan() {
         f32::NAN.to_bits() // Normalize all NaNs to a single bit pattern
      } else if self.0 == 0.0 {
         0.0f32.to_bits() // Normalize -0.0 to 0.0
      } else {
         self.0.to_bits()
      };
      state.write_u32(bits);
   }
}

impl Ord for SafeFloat {
   fn cmp(&self, other: &Self) -> std::cmp::Ordering {
      self.0.partial_cmp(&other.0).unwrap()
      // Safely unwraps since NaN is impossible
   }
}

pub fn mk_safe_float(f: &f32) -> SafeFloat { SafeFloat(f.clone()) }
pub fn as_float(s: &SafeFloat) -> f32 { s.0 }

// Usage:
// my_vector.sort_by_key(|item| SafeFloat(item.float_field));

// ----- TESTS -------------------------------------------------------

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
pub mod functional_tests {
   use super::*;
   use crate::{
      create_testing,
      err_utils::ErrStr,
      utils::{ composer, debug }
   };
   use paste::paste;

   const MIN: f32 = 1.1;
   const MAX: f32 = 9.7;

   fn nums() -> Vec<f32> { vec![2.2, MAX, 8.4, MIN] }
   fn sorter() -> impl Fn(Vec<f32>) -> Vec<f32> {
      move |v: Vec<f32>| {
         let mut w = v;
         w.sort_by_key(mk_safe_float);
         w
      }
   }

   create_testing!("num::floats");

   run_with!("safefloat", " sort_by_key", nums(), composer(debug, sorter()));

   mod tests {
      use super::*;

      #[test] fn test_sort_by_key_safe_float() {
         let mut n = nums();
         n.sort_by_key(mk_safe_float);
         assert_eq!(Some(&MIN), n.first());
         assert_eq!(Some(&MAX), n.last());
      }
   }
}

