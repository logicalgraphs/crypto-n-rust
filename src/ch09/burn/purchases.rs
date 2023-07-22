use book::{
   csv_utils::CsvWriter,
   list_utils::ht
};

use crate::entries::{Entry,OrderBook};

pub struct Purchase {
   pub token: String,
   pub quote: f32,
   pub amount: f32,
   pub remaining: f32
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

impl CsvWriter for Purchase {
   fn as_csv(&self) -> String {
      format!("{},{},{}", self.quote, self.amount, self.remaining)
   }
}
