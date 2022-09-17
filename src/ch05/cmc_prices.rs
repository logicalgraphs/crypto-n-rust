// given a filename, we parse the JSON and spit out the prices

// Easy-peasy!

mod utils;
use std::fs;

// serde_json was recommended via https://stackoverflow.com/questions/30292752/how-do-i-parse-a-json-file

extern crate serde;

fn usage() {
   println!("./cmc_prices <filename>");
   println!("\n\tReads <filename> and extracts tokens and their prices.");
}

fn main() {
   if let Some(filename) = utils::head(utils::get_args()) {
      let data = fs::read_to_string(filename).expect("Unable to read file");
      let json: serde_json::Value =
         serde_json::from_str(&data).expect("ooga-booga: no JSONa!");
      let btc = &json["data"][0];
      println!("BTC is {}", btc);
      let eth = &json["data"][1];
      println!("ETH is {}", eth);
   } else {
      usage();
   }
}
