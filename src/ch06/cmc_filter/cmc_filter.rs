// filters and prints only the held assets' prices

use std::collections::{HashMap,HashSet};

use book::{
   utils::get_args,
   file_utils::lines_from_file
};

use crypto::types::coins::{Coin,print_all_coins,read_csv_coin};

fn usage() {
   println!("\n./cmc_filter <prices CSV file> <held assets LSV file>");
   println!("\n\tprints CSV of prices of held assets only.");
}

fn main() {
   if let [prices, assets] = get_args().as_slice() {
      parse_then_filter(prices, assets);
   } else {
      usage();
   }
}

fn parse_then_filter(prices: &str, assets: &str) {
   let price_lines = lines_from_file(prices);
   let assetss = lines_from_file(assets);
   let mut assets_set = HashSet::new();
   assetss.iter().for_each(|c| insert_if(&mut assets_set, c.to_string()));
   file_report("prices", &price_lines);
   file_report("assets", &assetss);
   let mut mappo = process_csv_prices(&price_lines);
   mappo.retain(|key, _| assets_set.contains(key));
   let mut coins: Vec<Coin> = mappo.into_values().collect();
   coins.sort();
   print_all_coins(coins);
}

fn insert_if(assets: &mut HashSet<String>, coin: String) {
   match assets.get(&coin) {
      Some(_) => { },
      None    => { assets.insert(coin); }
   };
}

fn file_report<T>(file: &str, lines: &[T]) {
   println!("{} has {} lines", file, lines.len());
}

fn process_csv_prices(lines: &Vec<String>) -> HashMap<String, Coin> {
   let (_, rows) = lines.split_at(3);
   let mut mappus = HashMap::new();
   rows.iter().for_each(|line| process_price_line(&mut mappus, line));
   mappus
}

fn process_price_line(m: &mut HashMap<String, Coin>, line: &String) {
   if let Ok(coin) = read_csv_coin(line) {
      if !m.contains_key(&coin.symbol) {
         m.insert(coin.symbol.to_string(), coin);
      }
   } else {
      println!("Could not process line {}", line);
   }
}
