extern crate serde;

use serde::Deserialize;
use serde_json::from_str;

use book::{list_utils::ht, csv_utils::CsvWriter};

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

pub struct Purchase {
   token: String,
   quote: f32,
   amount: f32,
   remaining: f32
}

pub fn mk_purchase(tok: &str, amount: f32, m: f32, remaining: f32) -> Purchase {
   let quote = m / amount;
   let token = tok.to_string();
   Purchase { token, quote, amount, remaining }
}

pub fn report_purchase(token: &str, amt: f32, purchase: &Purchase) -> String {
   format!("From {amt} {token}, I bought {} {}, quote: {}{}",
           purchase.amount, purchase.token, purchase.quote,
           remainder(token, purchase.remaining))
}

fn remainder(token: &str, rem: f32) -> String {
   if rem <= 0.0 { "".to_string()
   } else { format!("; {rem} {token} remain") }
}

impl CsvWriter for Purchase {
   fn as_csv(&self) -> String {
      format!("{},{},{}", self.quote, self.amount, self.remaining)
   }
}

pub fn buy(book: &OrderBook, amount: f32) -> Purchase {
   buy1(&book.base, &book.asks, amount, 0.0, 0.0)
}

// ------ Purchase functions --------------------------------------------------

fn buy1(tok: &str, asks: &Vec<Entry>, remaining: f32, amount: f32, mult: f32)
   -> Purchase {
   if asks.is_empty() || remaining <= 0.0 {
      mk_purchase(tok, amount, mult, remaining)
   } else {
      if let (Some(entry), rest) = ht(asks) {
         let (quot, amt) = (entry.ratio, entry.amount);
         let bought = remaining.min(amt * quot);
         let new_rem = remaining - bought;
         let this_amount = bought / quot;
         buy1(tok, &rest, new_rem, amount + this_amount, mult + bought * quot)
      } else {
         panic!("Non-empty asks are empty!")
      }
   }
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
