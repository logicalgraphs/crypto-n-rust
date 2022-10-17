// types, regardless underlying data sources

use std::{
   fmt,
   num::ParseFloatError,
   str::FromStr
};

use book::list_utils::last;

#[derive(Debug, Clone)]
pub struct USD {
   amount: f32
}

impl PartialEq for USD {
   fn eq(&self, other: &Self) -> bool {
      self.amount == other.amount
   }
}

impl Eq for USD { }

pub fn mk_usd(amount: f32) -> USD {
   USD { amount }
}

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
