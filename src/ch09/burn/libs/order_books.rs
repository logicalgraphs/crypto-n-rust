extern crate serde;

use serde::Deserialize;
use serde_json::from_str;

use crate::libs::entries::Entry;

use book::{
   csv_utils::CsvWriter,
   err_utils::ErrStr,
   list_utils::ht
};

/*
An order book is:

ampLUNA/USK

asks

1.912,2.667917
1.916,2.665249
1.92,2.662584
1.925,5.319843
1.939,13.27301

bids

1.904,5.091939
1.9,5.086847
1.897,5.08176
1.891,10.153357
1.878,25.332626
*/

#[derive(Deserialize)]
struct Raw {
   #[serde(rename(deserialize="asks"))]
   asks1: Vec<Vec<String>>,
   #[serde(rename(deserialize="bids"))]
   bids1: Vec<Vec<String>>,
   #[serde(rename(deserialize="ticker_id"))]
   ticker_id1: String
}

#[derive(Debug, Clone)]
pub struct OrderBook {
   pub bids: Vec<Entry>,
   pub asks: Vec<Entry>,
   pub base: String,
   pub target: String
}

pub fn parse_orderbook(jsn: &str) -> ErrStr<OrderBook> {
   let raw: Raw = from_str(jsn).expect("RAW'd!");
   fn scan(section: &Vec<Vec<String>>) -> Result<Vec<Entry>, String> {
      section.iter().map(parse_v2e).collect()
   }
   let bids = scan(&raw.bids1)?;
   let asks = scan(&raw.asks1)?;
   let (base, target) = parse_bnt(&raw.ticker_id1)?;
   Ok(OrderBook{ bids, asks, base, target })
}

// ----- Printing functions ---------------------------------------------------

impl CsvWriter for OrderBook {
   fn as_csv(&self) -> String {
      let namei = format!("{}/{}", self.base, self.target);
      let a = thunk("asks", &self.asks, true);
      let b = thunk("bids", &self.bids, false);
      format!("{namei}\n\n{a}\n\n{b}")
   }
   fn ncols(&self) -> usize { 3 }
}

fn thunk(title: &str, section: &Vec<Entry>, revd: bool) -> String {
   let mut rows: Vec<String> = section.iter().map(|e| e.as_csv()).collect();
   if revd { rows.reverse(); }
   let rs = rows.join("\n");
   format!("{title}\n\n{rs}")
}

// ----- PARSING FUNCTIONS ---------------------------------------------------

fn parse_v2e(pair: &Vec<String>) -> Result<Entry, String> {
   if let (Some(rat), rest) = ht(pair) {
      let ratio: f32 = rat.parse()
          .expect(&format!("{rat} not a number for ratio!"));
      if let Some(amt) = rest.first() {
         let amount: f32 = amt.parse()
             .expect(&format!("{amt} not a number for amount!"));
         Ok(Entry { ratio, amount })
      } else {
        Err(format!("(ratio, amount) vector a singleton: {pair:?}"))
      }
   } else {
      Err("(ratio, amount) vector empty?!?".to_string())
   }
}

fn parse_bnt(tickr: &str) -> Result<(String, String), String> {
   let parts: Vec<&str> = tickr.split('_').collect();
   if let (Some(b), rest) = ht(&parts) {
      if let Some(t) = rest.first() {
         let a = if b == "STINJ" { "stINJ" } else { b };
         Ok((a.to_string(), t.to_string()))
      } else {
         Err(format!("bad split, ticker_id: {tickr}"))
      }
   } else {
      Err("ticker_id empty!".to_string())
   }
}
