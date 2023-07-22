use crate::{
   entries::OrderBook,
   purchases::Purchase
};

use book::utils::id;

use crypto::types::percentage::mk_percentage;

// ----- Printing functions/reportage -----------------------------------------

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
