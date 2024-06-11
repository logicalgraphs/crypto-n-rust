// extracts current market data from 
// https://api.kujira.app/api/coingecko/tickers

extern crate serde;

use serde::{Deserialize,Deserializer};
use serde_json::{Value, from_str};

use std::{
   collections::HashSet,
   hash::{Hash,Hasher}
};

use book::json_utils::unquot;

use crate::rest_utils::read_market_json;

#[derive(Debug, Clone)]
pub struct Book1 {
   // e.g.: {"ask":"1.8020000000","base_currency":"LUNA",
   //        "base_volume":"899.7562950000","bid":"1.7890000000",
   //        "high":"1.8709996622","last_price":"1.7890005387",
   //        "low":"1.7609999772",
   //        "pool_id":"kujira1yg8930mj8...p0kur",
   //        "target_currency":"axlUSDC","target_volume":"1647.8921550000",
   //        "ticker_id":"LUNA_axlUSDC"},

   pub base: String,
   pub target: String,
   pub pool_id: String,
   pub base_vol: f32,
   pub target_vol: f32,
   pub last: f32
}

pub type Books1 = HashSet<Book1>;

// ----- Parsing -------------------------------------------------------

#[derive(Deserialize)]
struct BooksVec {
   #[serde(rename(deserialize="tickers"))]
   books: Books1
}

pub async fn raw_books() -> Books1 {
   let str = read_market_json().await.expect("Could not read FIN market data");
   let books: BooksVec = from_str(&str).expect("booked!");
   books.books
}

// ----- impls -------------------------------------------------------

impl<'de> Deserialize<'de> for Book1 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
          where D: Deserializer<'de> {
      let json: Value = Value::deserialize(deserializer)?;
      let base = unquot(&json, "base_currency");
      let target = unquot(&json, "target_currency");
      let pool_id = unquot(&json, "pool_id");
      let lask1 = unquot(&json, "last_price");
      let last: f32 = lask1.parse().expect("last_price");
      let vol_raw2 = unquot(&json, "base_volume");
      let base_vol: f32 = vol_raw2.parse().expect("base vol");
      let vol_raw1 = unquot(&json, "target_volume");
      let target_vol: f32 = vol_raw1.parse().expect("target vol");
      Ok(Book1 { base, target, pool_id, base_vol, target_vol, last })
   }
}

impl Hash for Book1 {
   fn hash<H: Hasher>(&self, state: &mut H) {
      self.base.hash(state);
      self.target.hash(state);
      self.pool_id.hash(state);
   }
}

impl PartialEq for Book1 {
   fn eq(&self, other: &Self) -> bool {
      self.base == other.base
         && self.target == other.target
         && self.pool_id == other.pool_id
   }
}

impl Eq for Book1 {}
