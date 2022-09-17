// given a filename, we parse the JSON and spit out the prices

// Easy-peasy!

mod utils;
use std::{fs,fmt};

// serde_json was recommended via https://stackoverflow.com/questions/30292752/how-do-i-parse-a-json-file

extern crate serde;

use serde::{Deserialize,Deserializer};
use serde_json::Value;
// use serde_derive::Deserialize;

#[derive(Deserialize)]
struct Coins {
   #[serde(rename(deserialize="data"))]
   coins: Vec<Coin>
}

struct Coin {
   date: String,
   cmc_id: u32,
   rank: u32,
   name: String,
   symbol: String,
   price: Price
}

impl fmt::Display for Coin {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "{},{},{},{},{},{}",
              self.date,self.cmc_id,self.rank,self.symbol,self.name,self.price)
    }
}

struct Price {
   price: f32
}

impl fmt::Display for Price {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "${:.2}", self.price)
    }
}

impl<'de> Deserialize<'de> for Coin {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de> {
      let json: Value = Value::deserialize(deserializer)?;
      let quot: &Value = json.get("quote").expect("quote");
      let usd: &Value = quot.get("USD").expect("USD");
      let price = val_num::<f32>(usd, "price".to_string());
      let cmc_id = val_num::<u32>(&json, "id".to_string());
      let rank = val_num::<u32>(&json, "cmc_rank".to_string());
      let name = dequote(val_str(&json, &"name".to_string()));
      let symbol = dequote(val_str(&json, &"symbol".to_string()));
      let date = val_date(&json, &"last_updated".to_string());
      Ok(Coin { date, cmc_id, rank, name, symbol, price: Price { price } })
   }
}

fn val_str(val: &Value, idx: &String) -> String {
   val[idx].to_string()
}

fn val_num<T: std::str::FromStr>(val: &Value, idx: String) -> T
      where <T as std::str::FromStr>::Err: std::fmt::Debug {
   let val_str: String = val_str(val, &idx);
   val_str.parse().expect(&idx)
}

fn val_date(val: &Value, idx: &String) -> String {
   let mut date = dequote(val_str(val, idx));
   date.truncate(10);
   date
}

fn dequote(mut str: String) -> String {
   str.pop();
   str.remove(0);
   str
}

fn usage() {
   println!("./cmc_prices <filename>");
   println!("\n\tReads <filename> and extracts tokens and their prices.");
}

fn main() {
   if let Some(filename) = utils::head(utils::get_args()) {
      let data = fs::read_to_string(filename).expect("Unable to read file");
      let coins: Coins =
         serde_json::from_str(&data).expect("ooga-booga: no JSONa!");

      println!("There are {} coins.\n", coins.coins.len());

      print_header();

// or a special serializer, a la: https://serde.rs/stream-array.html

      coins.coins.iter().for_each(print_coin);
   } else {
      usage();
   }
}

fn print_header() {
   println!("date,cmc_id,rank,symbol,name,price");
}

fn print_coin(coin: &Coin) {
   println!("{}", coin);
}
