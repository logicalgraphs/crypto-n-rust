use std::{
   cmp::Ordering,
   hash::{Hash,Hasher}
};

use book::csv_utils::CsvWriter;

#[derive(Debug, Clone)]
pub struct Asset {
   token: String,
   amount: f32,
   quote: f32
}

impl CsvWriter for Asset {
   fn as_csv(&self) -> String { csv(self) }
}

impl Hash for Asset {
   fn hash<H: Hasher>(&self, state: &mut H) {
      self.token.hash(state);
   }
}

impl PartialEq for Asset {
   fn eq(&self, other: &Self) -> bool {
      self.token == other.token
   }
}

impl Eq for Asset {}

impl Ord for Asset {
   fn cmp(&self, other: &Self) -> Ordering {
      self.token.cmp(&other.token)
   }
}

impl PartialOrd for Asset {
   fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
      Some(self.cmp(other))
   }
}

fn csv(asset: &Asset) -> String {
   format_args!("{},{},{}",asset.token, asset.amount, asset.quote).to_string()
}

pub fn mk_asset(token: String, amount: f32, quote: f32) -> Asset {
   Asset { token, amount, quote }
}

pub fn parse_asset(tok: &str, amt: &str, quot: &str)
   -> Result<Asset, String> {
   let amount: f32 = amt.parse().expect("amount");
   let quote: f32 = quot.parse().expect("quote");
   let token = tok.to_string();
   Ok(Asset { token, amount, quote })
}

pub fn merge_assets(a1: &Asset, a2: Asset) -> Asset {
   let token = &a1.token;
   let amount = a1.amount + a2.amount;
   let quote = (a1.quote * a1.amount + a2.quote * a2.amount) / amount;
   mk_asset(token.to_string(), amount, quote)
}

pub fn read_csv_asset(line: &String) -> Result<Asset, String> {
   if let [token, amount, quote] =
         line.split(',').collect::<Vec<&str>>().as_slice() {
      parse_asset(token, amount, quote)
   } else {
      Err("Can't parse line: ".to_owned() + line)
   }
}
