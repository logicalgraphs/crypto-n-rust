use std::{
   collections::HashMap,
   fs::File,
};

extern crate serde;

use serde_json::from_reader;
use serde::Deserialize;

use book::{
   csv_utils::CsvWriter,
   utils::get_args
};

// answer06_local tests JSON-ingest with a local sample-file

/* 
entry in the token-prices dictionary:

"arbitrum":{"usd":0.961968}

Let's HashMap this mofo!
*/

#[derive(Deserialize, Debug, Clone)]
struct Quote { usd: f32 }

impl CsvWriter for Quote {
   fn as_csv(&self) -> String { format!("{}", self.usd) }
   fn ncols(&self) -> usize { 1 }
}

type TokenId = String;

fn prices(filename: &str) -> Vec<(TokenId,Quote)> {
   let file = File::open(filename)
       .expect("file should open read only");
   let recs: HashMap<TokenId,Quote> = from_reader(file).expect("JSON'd!");
   let mut ans: Vec<(TokenId, Quote)> = recs.into_iter().collect();
   ans.sort_by(|a,b| a.0.cmp(&b.0));
   ans
}

fn usage() {
   println!("\n./answer06_local <token-prices JSON filename>\n");
   println!("\tReads the JSON and converts it into token-prices HashMap.\n");
}

fn main() {
   let args = get_args();
   if let Some(filename) = args.first() {
      let p = prices(&filename);
      println!("token id,price");
      p.iter().for_each(|(token,price)| println!("{token},{}", price.as_csv()));
   } else {
      usage();
   }
}
