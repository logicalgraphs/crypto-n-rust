// algorithms for orders go here

use std::collections::HashSet;

use book::types::untag;

use crate::types::{
   assets::Asset,
   books::{fetch_books_by_vol,parse_books_with_aliases},
   interfaces::{Books,BookBooks,tokens,Prices,ticker,book_orderbook,vol_24h},
   marketplace::{OrderBook,dual_asset,orderbook},
   usd::USD,
   volumes::{Volumes,volumes_by_token}
};

pub fn target_sell_ratio(prices: &Prices, a: &Asset,
                         on: &OrderBook, perc: f32) -> Option<(String, f32)> {
   if on.ratio > 0.0 {
      let buy = dual_asset(on, a);
      prices.get(&buy).and_then(|buy_quote| {
         let quot = untag(&buy_quote).1;
         Some((buy.clone(), quot.amount / a.quote * perc))
      })
   } else { None }
}

pub fn active_order_books(market: &mut HashSet<OrderBook>,
                          tickers: &Books, vol: USD) {
   let winnow: HashSet<String> =
      fetch_books_by_vol(tickers, vol).iter().map(ticker).collect();
   market.retain(|b| winnow.contains(&orderbook(&b)));
}

pub fn working_set(min: f32, b: &Books) -> (Volumes, Books) {
   let mut books = b.clone();
   books.retain(|b| vol_24h(b).amount > min);
   let mut tok_vols = volumes_by_token(&b);
   let toks = tokens(&books);
   tok_vols.retain(|t, _| toks.contains(t));
   (tok_vols, books)
}

pub fn books_orderbooks((prices, books): &BookBooks) -> HashSet<OrderBook> {
   books.into_iter().map(book_orderbook(&prices)).collect()
}

pub async fn read_marketplace(date: &str) -> HashSet<OrderBook> {
   println!("Reading order books from FIN REST endpoint");
   let books = parse_books_with_aliases(&date).await;
   books_orderbooks(&books)
}
