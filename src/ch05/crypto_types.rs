// types, regardless underlying data sources

// serde_json was recommended via https://stackoverflow.com/questions/30292752/how-do-i-parse-a-json-file

extern crate serde;
              
use serde::{Deserialize,Deserializer};
use serde_json::Value;

use std::fmt;

use crate::json_utils::{val_str, val_num, val_date, dequote};
use crate::utils::last;

pub struct Coin {
   date: String,
   cmc_id: u32,
   rank: u32,
   name: String,
   symbol: String,
   price: USD
}

trait CsvReader {
   fn read(line: String) -> Result<Self, String> where Self: Sized;
}

trait CsvWriter {
   fn as_csv(&self) -> String;
}


impl<'de> Deserialize<'de> for Coin {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de> {
      let json: Value = Value::deserialize(deserializer)?;
      let quot: &Value = json.get("quote").expect("quote");
      let usd: &Value = quot.get("USD").expect("USD");
      let amount = val_num::<f32>(usd, "price".to_string());
      let cmc_id = val_num::<u32>(&json, "id".to_string());
      let rank = val_num::<u32>(&json, "cmc_rank".to_string());
      let name = dequote(val_str(&json, &"name".to_string()));
      let symbol = dequote(val_str(&json, &"symbol".to_string()));
      let date = val_date(&json, &"last_updated".to_string());
      Ok(Coin { date, cmc_id, rank, name, symbol, price: USD { amount } })
   }
}  

impl CsvWriter for Coin {
   fn as_csv(&self) -> String {
      format_args!("{},{},{},{},{},{}", 
              self.date,self.cmc_id,self.rank,self.symbol,self.name,self.price)
              .to_string()
   }
}

pub struct USD {
   amount: f32
}

impl fmt::Display for USD {
   fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
      write!(formatter, "${:.2}", self.amount)
   }
}

impl CsvReader for USD {
   fn read(elt: String) -> Result<USD, String> {
      if let Some(num) = last(elt.split('$').collect()) {
         let amount_res = num.parse();
         if let Ok(amount) = amount_res {
            Ok(USD { amount })
         } else {
            Err(elt.to_owned() + " isn't USD")
         }
      } else {
         Err("No monay on the empty string".to_string())
      }
   }
}

pub fn print_all_coins(coins: Vec<Coin>) {
   println!("There are {} coins.\n", coins.len());
   print_header();
   coins.iter().for_each(print_coin);
}

pub fn print_header() {
   println!("date,cmc_id,rank,symbol,name,price");
}

pub fn print_coin(coin: &Coin) {
   println!("{}", coin.as_csv());
}
