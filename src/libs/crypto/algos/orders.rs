// algorithms for orders go here

use std::collections::HashSet;

use crate::{
   rest_utils::graphs_fin_res,
   types::{
      assets::Asset,
      books::{Books,BookBooks,Prices,
              fetch_books_by_vol,ticker,book_orderbook,parse_books},
      marketplace::{OrderBook,dual_asset,orderbook},
      usd::USD
   }
};

pub fn target_sell_ratio(prices: &Prices, a: &Asset,
                         on: &OrderBook, perc: f32) -> Option<(String, f32)> {
   if on.ratio > 0.0 {
      let buy = dual_asset(on, a);
      prices.get(&buy).and_then(|buy_quote|
         Some((buy.clone(), buy_quote.amount / a.quote * perc)))
   } else { None }
}

pub fn active_order_books(market: &mut HashSet<OrderBook>,
                          tickers: &Books,
                          vol: USD) {
   let winnow: HashSet<String> =
      fetch_books_by_vol(tickers, vol).iter().map(ticker).collect();
   market.retain(|b| winnow.contains(&orderbook(&b)));
}

pub fn books_orderbooks((prices, books): &BookBooks) -> HashSet<OrderBook> {
   books.into_iter().map(book_orderbook(&prices)).collect()
}

pub fn read_marketplace() -> HashSet<OrderBook> {
   println!("Reading order books from FIN REST endpoint");
   books_orderbooks(&parse_books(Some(graphs_fin_res("aliases.csv"))))
}
