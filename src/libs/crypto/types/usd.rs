// types, regardless underlying data sources

use std::{
   cmp::Ordering,
   fmt,
   hash::{Hash,Hasher},
   num::ParseFloatError,
   str::FromStr
};

use book::{
   list_utils::last,
   num_utils::integer_decode
};

#[derive(Debug, Clone)]
pub struct USD {
   amount: f32,
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
      write!(formatter, "${:.2}", self.amount)
   }
}

impl FromStr for USD {
   type Err = ParseFloatError;

   fn from_str(elt: &str) -> Result<Self, Self::Err> {
      if let Some(num) = last(elt.split('$').collect()) {
         if let Ok(amount) = num.parse() {
            Ok(mk_usd(amount))
         } else {
            panic!("{} isn't a number", num)
         }
      } else {
        panic!("USD: empty string")
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
      Some(self.cmp(other))
   }
}

// ----- ... and our methods -------------------------------------------------

pub fn mk_usd(amount: f32) -> USD {
   let decode = integer_decode(amount.into());
   USD { amount, decode }
}
