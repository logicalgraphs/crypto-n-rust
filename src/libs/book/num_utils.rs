use std::{
   fmt,
   mem
};

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

pub fn parse_commaless(str: &str) -> Result<f32, String> {
   let mut no_comma = str.to_string();
   no_comma.retain(no(','));
   match no_comma.parse() {
      Ok(x) => Ok(x),
      Err(_) => Err(format!("{str} is not a number"))
   }
}

fn no(ch: char) -> impl Fn(char) -> bool {
   move |chr| chr != ch
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
