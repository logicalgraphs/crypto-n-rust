// given a filename, we parse the JSON and spit out the prices

// Easy-peasy!

use cmc_prices::csv_utils::print_all_coins;
use cmc_prices::utils::{head, get_args};
use cmc_prices::json_utils::parse_coins;

use std::fs;

fn usage() {
   println!("\n./cmc_prices <filename>");
   println!("\n\tReads <filename> and extracts tokens and their prices.\n");
}

fn main() {
   if let Some(filename) = head(get_args()) {
      let data = fs::read_to_string(filename).expect("Unable to read file");
      let coins = parse_coins(&data);
      print_all_coins(coins);
   } else {
      usage();
   }
}
