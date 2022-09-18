// given a filename, we parse the JSON and spit out the prices

// Easy-peasy!

use cmc_prices::crypto_types::{Coin, print_all_coins};
use cmc_prices::utils::{head, get_args};

use std::fs;

extern crate serde;

use serde::Deserialize;
use serde_json::from_str;

#[derive(Deserialize)]
struct Coins {
   #[serde(rename(deserialize="data"))]
   coins: Vec<Coin>
}

fn usage() {
   println!("\n./cmc_prices <filename>");
   println!("\n\tReads <filename> and extracts tokens and their prices.");
}

fn main() {
   if let Some(filename) = head(get_args()) {
      let data = fs::read_to_string(filename).expect("Unable to read file");
      let coins: Coins = from_str(&data).expect("ooga-booga: no JSONa!");
      print_all_coins(coins.coins);
   } else {
      usage();
   }
}
