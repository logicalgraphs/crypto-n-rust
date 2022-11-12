use std::{
   collections::HashSet
};

use book::{
   utils::get_args
};

use crypto::{
   types::{
      assets::Asset,
      marketplace::{OrderBook,read_marketplace,fetch_sell_books,orderbook},
      portfolio::{Portfolio,assets_from_file,print_portfolio,for_each_asset},
      usd::mk_usd
   },
   algos::orders::target_sell_ratio
};

fn usage() {
   println!("\n./rekt <assets CSV file> <marketplace LSV file>");
   println!("\n\tmakes bid rekt-omendations based on portfolio and markets.");
   println!("\n\t... if you blindly follow these bids, you get what's comin' t'ye.");
   println!("\n\tCaveat Emptor. DYOR. YMMV.");
}

fn main() {
   if let [assets_file, markets_file] = get_args().as_slice() {
      let markets = read_marketplace(markets_file);
      let portfolio = assets_from_file(assets_file);
      print_portfolio(&portfolio);

      println!("\nRecommendations\n");

      for_each_asset(&portfolio, |asset| rec(&portfolio, &markets, asset));
   } else {
      usage();
   }
}

fn rec(p: &Portfolio, m: &HashSet<OrderBook>, sell: &Asset) {
   let books = fetch_sell_books(m, sell);
   books.iter().for_each(|book| anal(p, sell, book));
}

fn anal(p: &Portfolio, sell: &Asset, book: &OrderBook) {
   if let Some(target) = target_sell_ratio(sell, book, 1.1) {
      println!("SELL {} on {} at {}", &sell.token, orderbook(book), target);
   }
}
