use std::{
   collections::HashMap,
   fs::File,
};

extern crate serde;

use serde_json::from_reader;
use serde::Deserialize;

use book::{
   csv_utils::CsvWriter,
   err_utils::ErrStr,
   utils::get_args
};

use swerve::{
   token_ids::Dict,
   snarf::snarf_pivots
};

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

fn fetch_prices(filename: &str) -> HashMap<TokenId,Quote> {
   let file = File::open(filename)
       .expect("file should open read only");
   let prices: HashMap<TokenId,Quote> = from_reader(file).expect("JSON'd!");
   prices
}

type Token = String;

fn prices(pric: HashMap<TokenId,Quote>, dict: &Dict) -> Vec<(Token,Quote)> {

/* Rust making borrowing in higher-order contexts atlasian
   fn firstM<'a, A: Clone>(f: impl Fn(&'a String) -> Option<&'a String>)
         -> impl FnMut((String, A)) -> Option<(String, A)> {
      move |tup| f(&tup.0).and_then(|x| Some((x.to_string(), tup.1.clone())))
   }
   fn dict_f<'a>(d: &'a Dict) -> impl Fn(&'a String) -> Option<&'a String> {
      move |a| d.get(a)
   }
*/
   let rows: HashMap<Token, Quote> =
      pric.into_iter()
          .filter_map(   // firstM(dict_f(&dict)))
                 |(k,v)| dict.get(&k).and_then(|x| Some((x.to_string(),v))))
                 // much easier with monads and arrows, seriously! :<
          .collect();
   let mut ans: Vec<(Token, Quote)> = rows.into_iter().collect();
   ans.sort_by(|a,b| a.0.cmp(&b.0));
   ans
}

fn usage() {
   println!("\n./answer06 <token-prices JSON filename>\n");
   println!("\tReads the JSON and converts it into token-prices HashMap.\n");
}

#[tokio::main]
async fn main() -> ErrStr<()> {
   let args = get_args();
   if let Some(filename) = args.first() {
      let (_, dict) = snarf_pivots().await?;
      let p = fetch_prices(&filename);
      let q = prices(p, &dict);
      println!("token,price");
      q.iter().for_each(|(token,price)| println!("{token},{}", price.as_csv()));
   } else {
      usage();
   }
   Ok(())
}
