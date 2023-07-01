extern crate serde;

use serde::Deserialize;
use serde_json::from_str;

use book::{list_utils::ht, csv_utils::CsvWriter};

#[derive(Debug, Clone)]
pub struct Entry {
   ratio: f32,
   amount: f32
}

impl CsvWriter for Entry {
   fn as_csv(&self) -> String { format!("{},{}", self.ratio, self.amount) }
}

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
   bids: Vec<Entry>,
   asks: Vec<Entry>,
   base: String,
   target: String
}

impl CsvWriter for OrderBook {
   fn as_csv(&self) -> String {
      let namei = format!("{}/{}", self.base, self.target);
      let a = thunk("asks", &self.asks);
      let b = thunk("bids", &self.bids);
      format!("{namei}\n\n{a}\n\n{b}")
   }
}

pub fn parse_orderbook(jsn: &str) -> Result<OrderBook, String> {
   let raw: Raw = from_str(jsn).expect("RAW'd!");
   let bids_p: Result<Vec<Entry>, String> =
      raw.bids1.iter().map(|e| parse_v2e(e)).collect();
   let asks_p: Result<Vec<Entry>, String> =
      raw.asks1.iter().map(|e| parse_v2e(e)).collect();
   let (base, target) = parse_bnt(&raw.ticker_id1)?;
   let bids = bids_p?;
   let asks = asks_p?;
   Ok(OrderBook{ bids, asks, base, target })
}

// ----- Printing functions --------------------------------------------------

fn thunk(title: &str, section: &Vec<Entry>) -> String {
   
   let rows: Vec<String> = section.iter().map(|e| e.as_csv()).collect();
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
         Ok((b.to_string(), t.to_string()))
      } else {
         Err(format!("bad split, ticker_id: {tickr}"))
      }
   } else {
      Err("ticker_id empty!".to_string())
   }
}
