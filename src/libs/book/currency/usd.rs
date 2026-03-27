// types, regardless underlying data sources

use std::{
   cmp::Ordering,
   fmt,
   iter::Sum,
   ops::{Add,AddAssign},
   str::FromStr
};

use crate::{
   err_utils::ErrStr,
   num_utils::parse_commaless
};

#[derive(Debug, Clone, PartialEq)]
pub struct USD {
   pub amount: f32
}

// ----- implementations -----------------------------------------------------

impl fmt::Display for USD {
   fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
      let a = self.amount;
      let b = a.abs();
      let sign = if a < 0.0 { "-" } else { "" };
      let precision =
         if b == 0.0 { 2
         } else if b < 0.01 { 6
         } else if b < 0.99 { 4
         } else { 2 };
      write!(formatter, "{sign}${:.*}", precision, b)
   }
}

impl FromStr for USD {
   type Err = String;

   fn from_str(elt: &str) -> ErrStr<Self> {
      let splitage: Vec<&str> = elt.split('$').collect();
      if let Some(num) = splitage.last() {
         let sgn =
            splitage.first()
                    .and_then(|s| Some(if s == &"-" { -1.0 } else { 1.0 }))
                    .or(Some(1.0))
                    .unwrap();
         let amount = parse_commaless(&num.to_string())?;
         Ok(mk_usd(sgn * amount))
      } else {
         Err(format!("USD: empty string"))
      }
   }
}

impl Eq for USD { }

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
   USD { amount }
}

pub fn no_monay() -> USD { mk_usd(0.0) }

// ----- TESTS -------------------------------------------------------

pub mod functional_tests {
   use super::*;
   use crate::err_utils::err_or;

   pub fn runoff() -> ErrStr<usize> {
      fn to_usd(s: &str) -> ErrStr<String> {
         let usd: USD =
            err_or(s.parse(), &format!("Unable to parse money {s}"))?;
         Ok(format!("{usd}"))
      }
      println!("\ncurrency::usd::fmt::Display functional test\n");
      println!("Price-quotes as of 2026-03-13:

ADA	AVAX	BTC		ETH		HBAR	UNDEAD");
      let qts: Vec<String> =
         vec!["$0.2716","$9.88","$71,427.00","$2,119.70","$0.0969","$0.002465"]
            .iter().filter_map(|s| to_usd(s).ok()).collect();
      println!("{}", qts.join("\t"));
      Ok(1)
   }
}

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
   fn test_parse_negative_amount() -> ErrStr<()> {
      let negger: USD = "-$9.30".parse()?;
      assert_eq!(-9.3, negger.amount);
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

