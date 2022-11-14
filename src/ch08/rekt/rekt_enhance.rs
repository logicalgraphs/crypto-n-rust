use std::{
   collections::{HashMap,HashSet}
};

use book::{
   utils::get_args
};

use crypto::{
   types::{
      assets::Asset,
      marketplace::{OrderBook,read_marketplace,fetch_sell_books,
                    orderbook,fetch_buy_books,prices},
      portfolio::{assets_from_file,print_portfolio,for_each_asset},
      usd::USD
   },
   algos::orders::target_sell_ratio
};

fn usage() {
   println!("\n./rekt <assets CSV file> <marketplace LSV file> [token_list]");
   println!("\n\twhere token_list [tok1 tok2...]");
   println!("\t\tanalyzes ONLY those tokens");
   println!("\t\tif token_list is empty, analyzes all tokens in assets");
   println!("\nMakes bid rekt-omendations based on portfolio and markets.");
   let follow = "... if you blindly follow these bids,";
   let comeuppance = "you get what's comin' t'ye!";
   println!("\n\t{} {}", follow, comeuppance);
   println!("\nCaveat Emptor. DYOR. YMMV.\n");
}

fn main() {
   if let [assets_file, markets_file] = get_args().as_slice() {
      let markets = read_marketplace(markets_file);
      let portfolio = assets_from_file(assets_file);
      let prx = prices(&markets);
      print_portfolio(&portfolio);
      println!("\nRecommendations\n");
      for_each_asset(&portfolio, |asset| rec(&markets, &prx, asset));
   } else {
      usage();
   }
}

fn rec(m: &HashSet<OrderBook>, prx: &HashMap<String, USD>, sell: &Asset) {
   fetch_sell_books(m, sell).iter()
         .for_each(|book| sell_analysis(prx, sell, book));
   fetch_buy_books(m, sell).iter()
         .for_each(|book| buy_analysis(prx, sell, book));
}

fn sell_analysis(prx: &HashMap<String, USD>, sell: &Asset, book: &OrderBook) {
   if let Some((buy, target)) = target_sell_ratio(prx, sell, book, 1.1) {
      println!("SELL {} on {} at Price({}): {:.4}",
               &sell.token, orderbook(book), buy, target);
   }
}

fn buy_analysis(prx: &HashMap<String, USD>, sell: &Asset, book: &OrderBook) {
   if let Some((buy, target)) = target_sell_ratio(prx, sell, book, 0.8) {
      let tok = &sell.token;
      println!("BUY {} by SELLing {} on {} at Price({}): {:.4}",
               buy, tok, orderbook(book), tok, 1.0 / target);
   }
}
