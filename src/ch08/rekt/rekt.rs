use std::{
   collections::HashSet,
   path::Path
};

use book::{
   file_utils::lines_from_file,
   utils::get_args
};

use crypto::types::{
   assets::Asset,
   marketplace::{OrderBook,parse_lines,fetch_orderbooks_for,dual_asset},
   portfolio::{Portfolio,assets_from_file,print_portfolio,for_each_asset,
               fetch_asset_named}
};

fn usage() {
   println!("\n./rekt <assets CSV file> <marketplace LSV file>");
   println!("\n\tmakes bid rekt-omendations based on portfolio and markets.");
   println!("\n\t... if you blindly follow these bids, you get what's comin' t'ye.");
   println!("\n\tCaveat Emptor. DYOR. YMMV.");
}

fn main() {
   if let [assets, markets_file] = get_args().as_slice() {
      let markets = parse_n_print(markets_file);
      let portfolio = assets_from_file(assets);
      print_portfolio(&portfolio);

      println!("\nRecommendations\n");

      for_each_asset(&portfolio, |asset| rec(&portfolio, &markets, asset));
/*
      let atom_books = fetch_orderbooks(markets, "ATOM".to_string());
      println!("The ATOM order books are:");
      for o in atom_books.iter() {
         println!("\t{}", o);
      }
*/
   } else {
      usage();
   }
}

fn rec(p: &Portfolio, m: &HashSet<OrderBook>, sell: &Asset) {
   let books = fetch_orderbooks_for(m, sell);
   books.iter().for_each(|book| anal(p, sell, book));
}

fn anal(p: &Portfolio, sell: &Asset, book: &OrderBook) {
   let buy_str = dual_asset(book, sell);
   let buy1 = buy_str.clone();
   match fetch_asset_named(p, buy_str) {
      None => { println!("No bid for {} / {}", sell.token, buy1); },
      Some(buy) => { println!("We have a pair {} / {}", sell.token, buy.token); }
   }
}

fn parse_n_print(file: impl AsRef<Path>) -> HashSet<OrderBook> {
   let lines = lines_from_file(file);
   let (_header, rows) = lines.split_at(3);
   let mut pairs = HashSet::new();
   parse_lines(1, &mut pairs, rows.to_vec());
   println!("From {} lines, I have {} order books", lines.len(), pairs.len());
   pairs
}
