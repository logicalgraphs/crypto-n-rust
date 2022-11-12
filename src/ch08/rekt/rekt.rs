use std::{
   collections::HashSet
};

use book::{
   utils::get_args
};

use crypto::{
   types::{
      assets::Asset,
      marketplace::{OrderBook,read_marketplace,fetch_sell_books,orderbook,
                    dual_asset},
      portfolio::{Portfolio,assets_from_file,print_portfolio,for_each_asset},
   },
   algos::orders::target_sell_ratio
};

fn usage() {
   println!("\n./rekt <assets CSV file> <marketplace LSV file>");
   println!("\n\tmakes bid rekt-omendations based on portfolio and markets.");
   let follow = "... if you blindly follow these bids,";
   let comeuppance = "you get what's comin' t'ye!";
   println!("\n\t{} {}", follow, comeuppance);
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

fn rec(_p: &Portfolio, m: &HashSet<OrderBook>, sell: &Asset) {
   let books = fetch_sell_books(m, sell);
   books.iter().for_each(|book| anal(sell, book));
}

fn anal(sell: &Asset, book: &OrderBook) {
   if let Some(target) = target_sell_ratio(sell, book, 1.1) {
      let buy = dual_asset(book, sell);
      println!("SELL {} on {} at Price({}): {}",
               &sell.token, orderbook(book), buy, target);
   }
}
