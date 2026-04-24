use crate::libs::{
   entries::Entry,
   order_books::OrderBook
};

use book::{
   csv_utils::CsvWriter,
   list_utils::ht
};

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

pub fn explode_purchase(p: &Purchase) -> (String, f32, f32, f32) {
   let Purchase { token, quote, amount, remaining } = p;
   (token.clone(), quote.clone(), amount.clone(), remaining.clone())
}

pub fn buy(book: &OrderBook, amount: f32) -> Purchase {
   purchase(&book.base, &book.asks, amount, 0.0, 0.0, true)
}

pub fn sell(book: &OrderBook, amount: f32) -> Purchase {
   purchase(&book.target, &book.bids, amount, 0.0, 0.0, false)
   // a sell is the dual of a buy, ... right? ;)
}

// ------ Purchase functions --------------------------------------------------

fn purchase(tok: &str, entries: &Vec<Entry>, remaining: f32, amount: f32,
            mult: f32, buy: bool) -> Purchase {
   if entries.is_empty() || remaining <= 0.0 {
      mk_purchase(tok, amount, mult, remaining)
   } else {
      if let (Some(entry), rest) = ht(entries) {
         let (quot, amt) = (entry.ratio, entry.amount);
         let bought = remaining.min(if buy { amt * quot } else { amt / quot });
         let new_rem = remaining - bought;
         let new_amount = amount
               + if buy { bought / quot } else { bought * quot };
         purchase(tok, &rest, new_rem, new_amount, mult + bought, buy)
      } else {
         let err = format!("Non-empty {} are empty!",
                           if buy { "asks" } else { "bids" });
         panic!("{err}")
      }
   }

}

// ----- Printing functions -----------------------------------------

impl CsvWriter for Purchase {
   fn as_csv(&self) -> String {
      format!("{},{},{}", self.quote, self.amount, self.remaining)
   }
   fn ncols(&self) -> usize { 3 }
}
