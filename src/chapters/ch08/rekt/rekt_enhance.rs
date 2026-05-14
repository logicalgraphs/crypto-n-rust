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
                    orderbook,fetch_buy_books,prices,prices_usk},
      portfolio::{assets_from_file,print_portfolio,for_each_asset,consider},
      usd::{USD,mk_usd}
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
   let args = get_args();
   if args.len() > 1 {
      let (files, toks) = args.split_at(2);
      if let [assets_file, markets_file] = files.to_vec().as_slice() {
         let markets = read_marketplace(markets_file);
         // println!("SHOW ME THE MARKETPLACES! {:?}", markets);
         let tokens: Vec<String> = toks.to_vec();
         let portfolio = consider(&assets_from_file(assets_file), &tokens);
         let prx_usk = prices_usk(&markets);
         // println!("prices_usk: {:?}", prx_usk);
         print_portfolio(&portfolio);
         println!("\nRecommendations\n");
         for_each_asset(&portfolio, |asset| rec_usk(&markets, &prx_usk, asset));
         let prx = prices(&markets);
         for_each_asset(&portfolio, |asset| rec(&markets, &prx, asset));
      }
   } else {
      usage();
   }
}

fn rec_usk(m: &HashSet<OrderBook>, prx_usk: &HashMap<String, f32>,
           sell: &Asset) {
   let mut prx: HashMap<String, USD> = HashMap::new();
   for (k, v) in prx_usk {
      prx.insert(k.clone(), mk_usd(*v));
   }
   // println!("Analyzing USK books: {:?}", prx);
   rec(m, &prx, sell);
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
   if let Some((buy, target)) = target_sell_ratio(prx, sell, book, 0.9) {
      let tok = &sell.token;
      println!("BUY {} by SELLing {} on {} at Price({}): {:.4}",
               buy, tok, orderbook(book), tok, 1.0 / target);
   }
}
