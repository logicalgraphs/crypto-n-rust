// given a filename, we parse the JSON and spit out the prices

// Easy-peasy!

use book::{
   utils::get_args,
   list_utils::head,
};

use crypto::{
   coins::print_all_coins,
   json_utils::parse_coins
};

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
