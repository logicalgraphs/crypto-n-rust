// algorithms for orders go here

use std::collections::HashMap;

use crate::types::{
   assets::Asset,
   marketplace::{OrderBook,dual_asset},
   usd::USD
};

pub fn target_sell_ratio(prices: &HashMap<String, USD>, a: &Asset,
                         on: &OrderBook, percent: f32) -> Option<(String, f32)> {
   let mut ans: Option<(String, f32)> = None;

   if on.ratio > 0.0 {
      let buy = dual_asset(on, a);
      if let Some(buy_quote) = prices.get(&buy) {
         ans = Some((buy.clone(), buy_quote.amount / a.quote * percent));
      }
   }
   ans
}
