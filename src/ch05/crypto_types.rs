// types, regardless underlying data sources

use std::{fmt,num::ParseFloatError,str::FromStr};

use crate::utils::last;

pub struct Coin {
   date: String,
   cmc_id: u32,
   rank: u32,
   name: String,
   symbol: String,
   price: USD
}

pub fn mk_coin(date: String, cmc_id: u32, rank: u32, name: String,
               symbol: String, amount: f32) -> Coin {
   Coin { date, cmc_id, rank, name, symbol, price: mk_usd(amount) }
}

pub fn csv(coin: &Coin) -> String {
   format_args!("{},{},{},{},{},{}",
      coin.date, coin.cmc_id, coin.rank, coin.symbol, coin.name, coin.price)
      .to_string()
}

pub struct USD {
   amount: f32
}

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
