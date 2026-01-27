// types, regardless underlying data sources

use std::{
   cmp::Ordering,
   fmt,
   hash::{Hash,Hasher},
   iter::Sum,
   ops::{Add,AddAssign},
   str::FromStr
};

use crate::{
   err_utils::ErrStr,
   num_utils::{integer_decode,parse_commaless}
};

#[derive(Debug, Clone, Copy)]
pub struct USD {
   pub amount: f32,
   decode: (u64, i16, i8)
}

// ----- implementations -----------------------------------------------------

impl PartialEq for USD {
   fn eq(&self, other: &Self) -> bool {
      self.amount == other.amount
   }
}

impl Eq for USD { }

impl fmt::Display for USD {
   fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
      let sign = if self.amount < 0.0 { "-" } else { "" };
      write!(formatter, "{sign}${:.2}", self.amount.abs())
   }
}

impl FromStr for USD {
   type Err = String;

   fn from_str(elt: &str) -> ErrStr<Self> {
      if let Some(num) = elt.split('$').collect::<Vec<_>>().last() {
         let amount = parse_commaless(&num.to_string())?;
         Ok(mk_usd(amount))
      } else {
         Err(format!("USD: empty string"))
      }
   }
}

impl Hash for USD {
   fn hash<H: Hasher>(&self, state: &mut H) {
      self.decode.hash(state);
   }
}

impl Ord for USD {
   fn cmp(&self, other: &Self) -> Ordering {
      let a = (self.amount * 100.0) as i32;
      let b = (other.amount * 100.0) as i32;
      a.cmp(&b)
   }
}

impl PartialOrd for USD {
   fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
      self.amount.partial_cmp(&other.amount)
   }
}

impl Add for USD {
   type Output = Self;
   fn add(self, rhs: USD) -> Self {
      mk_usd(self.amount + rhs.amount)
   }
}

impl AddAssign for USD {
   fn add_assign(&mut self, other: Self) {
      *self = mk_usd(self.amount + other.amount);
   }
}

// https://users.rust-lang.org/t/implementing-the-sum-trait/23332/3
// vitalyd answer, modified to value-impl by moiself (that is French).

impl Sum<Self> for USD {
    fn sum<I>(iter: I) -> Self
          where I: Iterator<Item = Self> {
       iter.fold(no_monay(), |a, b| mk_usd(a.amount + b.amount))
    }
}

// ----- ... and our methods -------------------------------------------------

pub fn mk_usd(amount: f32) -> USD {
   let decode = integer_decode(amount.into());
   USD { amount, decode }
}

pub fn no_monay() -> USD { mk_usd(0.0) }

// ----- TESTS -------------------------------------------------------

#[cfg(test)]
mod tests {
   use super::*;

   #[test]
   fn test_no_monay_is_zero() {
      assert_eq!(mk_usd(0.0), no_monay());
   }

   #[test]
   fn test_parse_ok() {
      let ans: ErrStr<USD> = "$5.29".parse();
      assert!(ans.is_ok());
   }

   #[test]
   fn test_parse_amount() -> ErrStr<()> {
      let fiver: USD = "$5".parse()?;
      assert_eq!(5.0, fiver.amount);
      Ok(())
   }

   #[test]
   fn test_sum() {
      let fiver = mk_usd(5.0);
      let tri = mk_usd(3.14);
      let sum = fiver + tri;
      assert_eq!(mk_usd(8.14), sum);
   }

   #[test]
   fn test_parse_commaless() {
      let ans: ErrStr<USD> = "$89,534.12".parse();
      assert!(ans.is_ok());
   }

   #[test]
   fn test_ordering() {
      let btc_quote = mk_usd(88094.0);
      let eth_quote = mk_usd(2923.35);

      // quotes from 2026-01-27

      assert!(btc_quote > eth_quote);
   }
}

