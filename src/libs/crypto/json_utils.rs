// common utils for deserializing JSON

// the serde-like aspect of our Coin-type

// serde_json was recommended via https://stackoverflow.com/questions/30292752/how-do-i-parse-a-json-file

use book::json_utils::{unquot,val_num,val_date};
use crate::types::coins::{Coin, mk_coin};

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
      let amount = val_num::<f32>(usd, "price").expect("price");
      let cmc_id = val_num::<u32>(&json, "id").expect("id");
      let rank = val_num::<u32>(&json, "cmc_rank").expect("cmc_rank");
      let name = unquot(&json, "name");
      let symbol = unquot(&json, "symbol");
      let date = val_date(&json, "last_updated").expect("last_updated");
      Ok(mk_coin(date, cmc_id, rank, name, symbol, amount))
   }
}
