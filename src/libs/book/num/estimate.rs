use std::{fmt,str::FromStr};

use crate::err_utils::ErrStr;

// --- Estimates -------------------------------------------------------

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

impl FromStr for Estimate {
   type Err = String;
   fn from_str(elt: &str) -> ErrStr<Self> {
      let ans = parse_estimate(elt)?;
      Ok(mk_estimate(ans))
   }
}

#[derive(Debug, Clone)]
pub struct Estimate {
   pub val: f32
}

pub fn mk_estimate(val: f32) -> Estimate {
   Estimate { val }
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
