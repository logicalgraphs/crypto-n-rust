// types, regardless underlying data sources

use std::{
   cmp::Ordering,
   fmt,
   hash::{Hash,Hasher},
   str::FromStr
};

use book::{
   list_utils::last,
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
      write!(formatter, "{sign}${:.3}", self.amount.abs())
   }
}

impl FromStr for USD {
   type Err = String;

   fn from_str(elt: &str) -> Result<Self, Self::Err> {
      if let Some(num) = last(elt.split('$').collect()) {
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
      self.decode.cmp(&other.decode)
   }
}

impl PartialOrd for USD {
   fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
      self.amount.partial_cmp(&other.amount)
   }
}

// ----- ... and our methods -------------------------------------------------

pub fn mk_usd(amount: f32) -> USD {
   let decode = integer_decode(amount.into());
   USD { amount, decode }
}

pub fn no_monay() -> USD { mk_usd(0.0) }

// Monoid on Add
pub fn sum_usd(a: &USD, b: &USD) -> USD {
   mk_usd(a.amount + b.amount)
}
