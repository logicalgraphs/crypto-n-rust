use std::{fmt,mem};

use crate::err_utils::{ErrStr,err_or};

// ----- parsers -------------------------------------------------------

pub fn parse_or(n_opt: Option<&String>, default: f32) -> f32 {
   n_opt.and_then(|n| n.parse().ok()).unwrap_or(default)
}

pub fn parse_num(s: &str) -> ErrStr<f32> {
   err_or(s.parse(), &format!("{s} is not a number"))
}

// for when we wish to treat blanks (e.g.) in spreadsheets as 0.0
pub fn parse_num_or_zero(s: &str) -> ErrStr<f32> {
   if s == "" { Ok(0.0) } else { parse_num(s) }
}

pub fn parse_commaless(str: &str) -> ErrStr<f32> {
   let mut no_comma = str.to_string();
   no_comma.retain(no(','));
   parse_num(&no_comma)
}

fn no(ch: char) -> impl Fn(char) -> bool {
   move |chr| chr != ch
}

// --- minimax -------------------------------------------------------

pub fn sort_f32(v: &mut Vec<f32>) {
   v.sort_by(|a,b| a.partial_cmp(&b).unwrap());
}

pub fn minimax_f32(v: &Vec<f32>) -> (Option<f32>, Option<f32>) {
   let mut srtd = v.clone();
   sort_f32(&mut srtd);
   (srtd.first().cloned(), srtd.last().cloned())
}

// --- Estimates -------------------------------------------------------

pub fn parse_estimate(str: &str) -> Result<f32, String> {
   match str.parse() {
      Ok(x) => Ok(x),
      Err(_) => {
         let mut playah = str.to_string();
         let mut err = true;
         let mut ans: f32 = 0.0;
         if let Some(mult) = playah.pop() {
            let mb_num: Result<f32, _> = playah.parse();
            if let Ok(num) = mb_num {
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
         }
         if err {
           Err(format!("Cannot derive estimate from {str}"))
         } else {
           Ok(ans)
         }
      }
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

// for when we need to serialize or to hash an f64 value

pub fn integer_decode(val: f64) -> (u64, i16, i8) {
   let bits: u64 = unsafe { mem::transmute(val) };
   let sign: i8 = if bits >> 63 == 0 { 1 } else { -1 };
   let exponent: i16 = ((bits >> 52) & 0x7ff) as i16;
   let seive: u64 = bits & 0xfffffffffffff;
   let mantissa = match exponent {
       0 => seive << 1,
       _ => seive | 0x10000000000000
   };
   (mantissa, exponent - (1023 + 52), sign)
}

