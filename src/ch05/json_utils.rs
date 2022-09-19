// common utils for deserializing JSON

// the serde-like aspect of our Coin-type

// serde_json was recommended via https://stackoverflow.com/questions/30292752/how-do-i-parse-a-json-file

use crate::string_utils::dequote;
use crate::crypto_types::{Coin, mk_coin};

extern crate serde;

use serde::{Deserialize,Deserializer};
use serde_json::{Value, from_str};

#[derive(Deserialize)]
pub struct Coins {
   #[serde(rename(deserialize="data"))]
   coins: Vec<Coin>
}

pub fn parse_coins(str: &String) -> Vec<Coin> {
   let coins: Coins = from_str(str).expect("ooga-booga: no JSONa!");
   coins.coins
}

impl<'de> Deserialize<'de> for Coin {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
          where D: Deserializer<'de> {
      let json: Value = Value::deserialize(deserializer)?;
      let quot: &Value = json.get("quote").expect("quote");
      let usd: &Value = quot.get("USD").expect("USD");
      let amount = val_num::<f32>(usd, "price".to_string());
      let cmc_id = val_num::<u32>(&json, "id".to_string());
      let rank = val_num::<u32>(&json, "cmc_rank".to_string());
      let name = dequote(val_str(&json, &"name".to_string()));
      let symbol = dequote(val_str(&json, &"symbol".to_string()));
      let date = val_date(&json, &"last_updated".to_string());
      Ok(mk_coin(date, cmc_id, rank, name, symbol, amount))
   }
}

pub fn val_str(val: &Value, idx: &String) -> String {
   val[idx].to_string()
}

pub fn val_num<T: std::str::FromStr>(val: &Value, idx: String) -> T
      where <T as std::str::FromStr>::Err: std::fmt::Debug {
   let val_str: String = val_str(val, &idx);
   val_str.parse().expect(&idx)
}

pub fn val_date(val: &Value, idx: &String) -> String {
   let mut date = dequote(val_str(val, idx));
   date.truncate(10);
   date
}
