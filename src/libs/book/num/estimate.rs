use std::{fmt,str::FromStr};

use crate::{ err_utils::ErrStr, types::values::Value, utils::pred };

// --- Estimates -----------------------------------------------------------

#[derive(Debug, Clone)]
pub struct Estimate {
   val: f32
}

pub fn mk_estimate(val: f32) -> Estimate {
   Estimate { val }
}

// ----- Implementations --------------------------------------------------

impl Estimate {
   pub fn approximates(&self, n: f32) -> bool {
      let base = self.val;
      let e = base * 0.05;
      base - e < n && n < base + e
   }

   pub fn is(&self, n: f32) -> ErrStr<()> {
      pred(self.approximates(n), ())
         .ok_or(format!("{n} is not proximate to {self}"))
   }
}

impl Value<f32> for Estimate { fn value(&self) -> f32 { self.val } }

impl FromStr for Estimate {
   type Err = String;
   fn from_str(elt: &str) -> ErrStr<Self> {
      let ans = parse_estimate(elt)?;
      Ok(mk_estimate(ans))
   }
}

pub fn parse_estimate(str: &str) -> ErrStr<f32> {
   match str.parse() {
      Ok(x) => Ok(x),
      Err(_) => {
         let mut playah = str.to_string();
         let mut err = true;
         let mut ans: f32 = 0.0;
         if let Some(mult) = playah.pop() {
            let num: f32 =
               playah.parse()
                     .or_else(|err|
                      Err(format!("Error parsing estimate {playah}: {err}")))?;
            let mult_up = mult.to_ascii_uppercase();
            if mult_up == 'K' {
               ans = num * 1000.0;
               err = false;
            }
            if mult_up == 'M' {
               ans = num * 1000000.0;
               err = false;
            }
         }
         if err {
           Err(format!("Cannot derive estimate from {str}"))
         } else {
           Ok(ans)
         }
      }
   }
}

impl fmt::Display for Estimate {
   fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
      let (mantissa, exponent) = match self.val {
         v if v.abs() > 999999.99 => (v/1000000.0, "M"),
         v if v.abs() > 999.99    => (v/1000.0,    "K"),
         v                        => (v, "")
      };
      write!(formatter, "{mantissa:.2}{exponent}")
   }
}

// ----- TESTS --------------------------------------------------------------

#[cfg(test)]
#[cfg(not(tarpaulin_include))]
mod sample_estimates {
   use super::*;
   use crate::{
      list_utils::filter_map_or,
      string_utils::words
   };

   pub fn some_nums() -> Vec<String> { words("15.53 1247.9 5614723.99") }
   pub fn some_ests() -> ErrStr<Vec<Estimate>> {
      filter_map_or(str::parse,
                    some_nums().iter().map(String::as_str).collect())
   }
}

#[cfg(test)]
#[cfg(not(tarpaulin_include))]
mod functional_tests {
   use super::*;
   use super::sample_estimates::*;
   use paste::paste;
   use crate::create_testing;

   create_testing!("num::estimates");


   run!("parse_estimate", {
      let ests = some_ests()?;
      println!("parsed estimates:\t{:?}", ests);
   });

   run!("display_estimate", {
      let strs: Vec<String> =
         some_ests()?.iter().map(|s| format!("{s}")).collect();
      println!("estimates:\t{}", strs.join(", "));
   });
}

#[cfg(test)]
#[cfg(not(tarpaulin_include))]
mod tests {
   use super::*;

   #[test] fn fail_parse_estimate() {
      let ans: ErrStr<Estimate> = "123adfkl".parse();
      assert!(ans.is_err());
   }

   #[test] fn test_parse_estimate_ok() {
      let ans: ErrStr<Estimate> = "15.35".parse();
      assert!(ans.is_ok());
      assert_eq!(15.35, ans.unwrap().val);
   }

   #[test] fn test_parse_estimate_k() -> ErrStr<()> {
      let ans: Estimate = "12k".parse()?;
      let est = ans.val;
      assert!(11000.0 < est && est < 13000.0);
      Ok(())
   }

   #[test] fn test_parse_estimate_m() -> ErrStr<()> {
      let ans: Estimate = "592.97M".parse()?;
      let val = 592974111.0;
      assert!(ans.approximates(val));
      Ok(())
   }

   #[test] fn test_approximates_exact() {
      let est = mk_estimate(15.35);
      assert!(est.approximates(15.35));
   }

   #[test] fn test_approximates_k() {
      let est = mk_estimate(54627.0);
      assert!(est.approximates(54630.0));
   }

   #[test] fn test_approximates_m() {
      let est = mk_estimate(123456789.0);
      assert!(est.approximates(120.0 * 1e06));
   }
}
