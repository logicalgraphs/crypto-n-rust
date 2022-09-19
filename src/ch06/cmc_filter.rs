// filters and prints only the held assets' prices

use std::path::Path;
use std::collections::HashMap;

use cmc_filter::utils::get_args;
use cmc_filter::file_utils::lines_from_file;
use cmc_filter::crypto_types::Coin;
use cmc_filter::csv_utils::read_csv_coin;

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

fn parse_then_filter(prices: impl AsRef<Path>, assets: impl AsRef<Path>) {
   let price_lines = lines_from_file(prices);
   let assetss = lines_from_file(assets);
   file_report("prices", &price_lines);
   file_report("assets", &assetss);
   let mappo = process_csv_prices(price_lines);
   if let Some(btc) = mappo.get("BTC") {
      println!("BTC is {:?}", btc);
   } else {
      for (k, v) in &mappo {
         println!("{}: {:?}", k, v);
      }
      panic!("Can't find BTC!");
   }
}

fn file_report<T>(file: &str, lines: &[T]) {
   println!("{} has {} lines", file, lines.len());
}

fn process_csv_prices(mut lines: Vec<String>)
      -> HashMap<String, Coin> {
   lines.remove(0);
   lines.remove(0);
   lines.remove(0);
   let mut mappus = HashMap::new();
   lines.iter().for_each(|line| process_price_line(&mut mappus, line));
   mappus
}

fn process_price_line(m: &mut HashMap<String, Coin>, line: &String) {
   if let Ok(coin) = read_csv_coin(line) {
      m.insert(coin.symbol.to_string(), coin);
   } else {
      println!("Could not process line {}", line);
   }
}
