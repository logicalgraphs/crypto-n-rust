// algorithms for orders go here

use std::collections::{HashMap,HashSet};

use crate::types::{
   assets::Asset,
   books::{Book,fetch_books_by_vol,ticker},
   marketplace::{OrderBook,dual_asset,orderbook},
   usd::USD
};

pub fn target_sell_ratio(prices: &HashMap<String, USD>, a: &Asset,
                         on: &OrderBook, perc: f32) -> Option<(String, f32)> {
   let mut ans: Option<(String, f32)> = None;

   if on.ratio > 0.0 {
      let buy = dual_asset(on, a);
      if let Some(buy_quote) = prices.get(&buy) {
         ans = Some((buy.clone(), buy_quote.amount / a.quote * perc));
      }
   }
   ans
}

pub fn active_order_books(market: &mut HashSet<OrderBook>,
                          tickers: &HashSet<Book>,
                          vol: f32) {
   let winnow: HashSet<String> =
      fetch_books_by_vol(tickers, vol).iter().map(ticker).collect();
   market.retain(|b| winnow.contains(&orderbook(&b)));
}
