#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct SafeFloat(f32);

impl Eq for SafeFloat {}

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

// from:

// https://www.google.com/search?q=sort_by+a+property+of+a+struct+rust&gs_lcrp=EgZjaHJvbWUyCQgAEEUYORigATIHCAEQIRigATIHCAIQIRiPAjIHCAMQIRiPAtIBCDk0OTVqMGo3qAIAsAIA&sourceid=chrome&ie=UTF-8&udm=50&fbs=ADc_l-aN0CWEZBOHjofHoaMMDiKpmAsnXCN5UBx17opt8eaTXyCfNeKGeJOJfUwi1MTUzwST1daiRLNBoQ0B6uczm6CPpE9MgkRBvAPuob4mewNkDkyJPOM4cUXCgTF3s6r8ePzSE49wWrN49hZO6o_W8OU5vZLEjpGBg_Hqzsox3WGwOApedjqloS2Uk0tumYhxMQS6X5e7CmdvlbFfLbas18Oj8ZlWFg&aep=10&ntc=1&mstk=AUtExfAZftEqqGWA8FZMPeR8ui_wZNHfqc1TrTtViNunNjUAS6l8AJq4-FNChjPYlkpZiOgroJcdoimhM8mpkqPu9TG4vdJBscK6afSs1Bpmlm6Vj2dmtUWQAPMHBlDLvl2VlUzZB8iLu1ylp2lg3ZO1t47K9oTkCMRD4KkgVhPsxclvvmNw20G_yawOXbb_QUfIj8V706iKaZ-Bkohxi2i4AN5u8wEwUgvaOPqEjQMctTKLj4vWCnqHReBEhiB4oRx2eSxGVuJj68iM1Q&aioh=3&csuir=1&mtid=sUYFap6_AanaptQPp--eoQY
// section 2: the Zero-Dependency Way (custom wrapper)

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

