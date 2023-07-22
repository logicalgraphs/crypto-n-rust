use std::fmt::{Result,Display,Formatter};

use book::{
   csv_utils::CsvWriter,
   list_utils::ht
};

use crate::entries::{Entry,OrderBook};

pub struct Purchase {
   call: Call,
   pub token: String,
   pub quote: f32,
   pub amount: f32,
   pub remaining: f32
}

pub fn mk_purchase(call: Call, tok: &str, amount: f32, m: f32, remaining: f32)
   -> Purchase {
   let quote = m / amount;
   let token = tok.to_string();
   Purchase { call, token, quote, amount, remaining }
}

pub enum Call { BUY, SELL }

pub fn trade(call: &Call, book: &OrderBook, amount: f32) -> Purchase {
   match call {
      Call::BUY => buy(book, amount),
      Call::SELL => sell(book, amount)
   }
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
      mk_purchase(Call::BUY, tok, amount, mult, remaining)
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
      mk_purchase(Call::SELL, tok, amount, mult, remaining)
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

impl Display for Call {
   fn fmt(&self, f: &mut Formatter<'_>) -> Result {
      write!(f, "{}", match self { Call::BUY => "BUY", _ => "SELL" } )
   }
}

impl CsvWriter for Call {
   fn as_csv(&self) -> String {
      format!("{}", self.to_string())
   }
}

impl CsvWriter for Purchase {
   fn as_csv(&self) -> String {
      format!("{},{},{},{}",
              self.call.as_csv(), self.quote, self.amount, self.remaining)
   }
}
