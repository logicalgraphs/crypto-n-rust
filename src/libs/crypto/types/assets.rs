use std::{
   cmp::Ordering,
   collections::HashSet,
   hash::{Hash,Hasher}
};

use book::csv_utils::{CsvWriter,print_csv};
use crate::types::{
   liquidations::{Liquidation,mk_liquidation},
   percentage::Percentage,
   usd::{USD,mk_usd}
};

#[derive(Debug, Clone)]
pub struct Asset {
   pub token: String,
   amount: f32,
   pub quote: f32
}

// ----- implementations ---------------------------------------------

impl Asset {
   pub fn liquidated_at(&self, p: Percentage) -> Liquidation {
      mk_liquidation(mk_usd(self.amount * self.quote), p)
   }
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

// ----- io -------------------------------------------------------

fn csv(asset: &Asset) -> String {
   let quot = mk_usd(asset.quote);
   format_args!("{},{},{}",asset.token, asset.amount, quot).to_string()
}

pub fn proto(token: String) -> Asset {
   mk_asset(token, 0.0, 0.0)
}

pub fn mk_asset(token: String, amount: f32, quote: f32) -> Asset {
   Asset { token, amount, quote }
}

pub fn parse_asset(tok: &str, amt: &str, quot: &str)
   -> Result<Asset, String> {
   let amount: f32 = amt.parse()
         .expect(&("not amount: ".to_owned() + &amt.to_owned()));
   let quot1: USD = quot.parse()
         .expect(&("not quote: ".to_owned() + &quot.to_owned()));
   Ok(mk_asset(tok.to_string(), amount, quot1.amount))
}

pub fn read_csv_asset(line: &String) -> Result<Asset, String> {
   if let [token, amount, quote] =
         line.split(',').collect::<Vec<&str>>().as_slice() {
      parse_asset(token, amount, quote)
   } else {
      Err("Can't parse line: ".to_owned() + line)
   }
}

pub fn read_assets(lines: Vec<String>) -> HashSet<Asset> {
   let mut bag = HashSet::new();
   let mut counter: i32 = 0;
   for line in lines {
      match read_csv_asset(&line) {
         Ok(asset) => { bag.insert(asset); counter += 1; },
         Err(msg) => { println!("Error: {}", msg); }
      }
   }
   println!("Parsed {} assets.", counter);
   bag
}

pub fn print_assets(assets: &HashSet<Asset>) {
   println!("asset,amount,quote");
   let mut bassets: Vec<&Asset> = assets.into_iter().collect();
   bassets.sort();
   bassets.iter().for_each(|ass| println!("{}", ass.as_csv()));
}

pub fn print_asset_d(bag: &HashSet<Asset>, a: &Asset, debug: bool) {
   if debug {
      if let Some(a1) = bag.get(a) {
         print_csv(a1);
      }
   }
}        

// ----- monoid -------------------------------------------------------

pub fn merge_assets(a1: &Asset, a2: Asset) -> Asset {
   let token = &a1.token;
   let amount = a1.amount + a2.amount;
   let quote = (a1.quote * a1.amount + a2.quote * a2.amount) / amount;
   mk_asset(token.to_string(), amount, quote)
}

pub fn split_asset(a1: &Asset, a2: Asset) -> Option<Asset> {
   let token = &a1.token;
   let amount = a1.amount - a2.amount;
   if amount <= 0.0 {
      None
   } else {
      let quote = (a1.quote * a1.amount - a2.quote * a2.amount) / amount;
      Some(mk_asset(token.to_string(), amount, quote))
   }
}

pub fn diff_usd(a1: &Asset, a2: &Asset) -> USD {
   mk_usd(a2.amount * a2.quote - a2.amount * a1.quote)
}

pub fn ratio_pair(a1: &Asset, a2: &Asset) -> (f32, f32) {
   (a1.quote / a2.quote, a2.quote / a1.quote)
}

// ----- hash-set operations ---------------------------------------------

pub fn add_asset(assets: &mut HashSet<Asset>, a: Asset) {
   assets.replace(match assets.get(&a) {
      Some(d) => merge_assets(d, a),
      None    => a
   });
}

pub fn remove_asset(assets: &HashSet<Asset>, a: Asset) -> HashSet<Asset> {
   match assets.get(&a) {
      Some(c) => {
         match split_asset(&c, a) {
            Some(d) => { replace_asset_with(assets, &d) },
            None    => { delete_asset(assets, &c) }
         }
      }
      None    => { assets.clone() }
   }
}

// -- helper functions for hash-set operation

fn replace_asset_with(assets: &HashSet<Asset>, d: &Asset) -> HashSet<Asset> {
   let mut ans = HashSet::new();
   for ass in assets.iter() {
      ans.insert(if d == ass { d.clone() } else { ass.clone() });
   }
   ans
}

fn delete_asset(assets: &HashSet<Asset>, c: &Asset) -> HashSet<Asset> {
   let mut ans = HashSet::new();
   for ass in assets.iter() {
      if c != ass { ans.insert(ass.clone()); }
   }
   ans
}
