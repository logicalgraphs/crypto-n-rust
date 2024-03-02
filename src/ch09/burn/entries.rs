extern crate serde;

use serde::Deserialize;
use serde_json::from_str;

use book::{
   csv_utils::CsvWriter,
   list_utils::ht,
   utils::id
};

use crypto::types::percentage::mk_percentage;

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
   pub base: String,
   pub target: String
}

pub fn parse_orderbook(jsn: &str) -> Result<OrderBook, String> {
   let raw: Raw = from_str(jsn).expect("RAW'd!");
   fn scan(section: &Vec<Vec<String>>) -> Result<Vec<Entry>, String> {
      section.iter().map(parse_v2e).collect()
   }
   let bids = scan(&raw.bids1)?;
   let asks = scan(&raw.asks1)?;
   let (base, target) = parse_bnt(&raw.ticker_id1)?;
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

pub fn buy(book: &OrderBook, amount: f32) -> Purchase {
   buy1(&book.base, &book.asks, amount, 0.0, 0.0)
}

pub fn sell(book: &OrderBook, amount: f32) -> Purchase {
   sell1(&book.target, &book.bids, amount, 0.0, 0.0)
   // a sell is the dual of a buy, ... right? ;)
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
         let new_amount = amount + bought / quot;
         buy1(tok, &rest, new_rem, new_amount, mult + bought)
      } else {
         panic!("Non-empty asks are empty!")
      }
   }
}

fn sell1(tok: &str, bids: &Vec<Entry>, remaining: f32, amount: f32, mult: f32)
   -> Purchase {
   if bids.is_empty() || remaining <= 0.0 {
      mk_purchase(tok, amount, mult, remaining)
   } else {
      if let (Some(entry), rest) = ht(bids) {
         let (quot, amt) = (entry.ratio, entry.amount);
         let bought = remaining.min(amt / quot);
         let new_rem = remaining - bought;
         let new_amount = amount + bought * quot;
         sell1(tok, &rest, new_rem, new_amount, mult + bought)
      } else {
         panic!("Non-empty bids are empty!")
      }
   }
}

// ----- Printing functions/reportage -----------------------------------------

impl CsvWriter for Entry {
   fn as_csv(&self) -> String { format!("{},{}", self.ratio, self.amount) }
   fn ncols(&self) -> usize { 2 }
}

impl CsvWriter for OrderBook {
   fn as_csv(&self) -> String {
      let namei = format!("{}/{}", self.base, self.target);
      let a = thunk("asks", &self.asks, true);
      let b = thunk("bids", &self.bids, false);
      format!("{namei}\n\n{a}\n\n{b}")
   }
   fn ncols(&self) -> usize { 3 }
}

pub fn report_sale(book: &OrderBook, amt: f32, purchase: &Purchase) -> String {
   report_purchase(&book.base, amt, purchase, true)
}

pub fn report_buy(book: &OrderBook, amt: f32, purchase: &Purchase) -> String {
   report_purchase(&book.target, amt, purchase, false)
}

fn report_purchase(token: &str, amt: f32, purchase: &Purchase, invert: bool)
   -> String {
   let quot_fn = if invert { |x: &f32| 1.0 / *x } else { id };
   format!("From {amt} {token}, I bought {} {}, quote: {}{}",
           purchase.amount, purchase.token, quot_fn(&purchase.quote),
           remainder(token, purchase.remaining))
}

pub fn report_roi(rate: f32, burn: f32, purchase: &Purchase) -> String {
   let quot = purchase.quote;
   let roi = (rate - quot) / quot;
   let apr = mk_percentage(roi * 365.0 / burn);
   format!("Burn ROI: {}, annualized to {apr}", mk_percentage(roi))
}

fn remainder(token: &str, rem: f32) -> String {
   if rem <= 0.0 { "".to_string()
   } else { format!("; {rem} {token} remain") }
}

impl CsvWriter for Purchase {
   fn as_csv(&self) -> String {
      format!("{},{},{}", self.quote, self.amount, self.remaining)
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
