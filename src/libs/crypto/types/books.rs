// extracts current market data from https://api.kujira.app/api/coingecko/tickers

extern crate serde;

use serde::{Deserialize,Deserializer};
use serde_json::{Value, from_str};

use std::{
   collections::HashSet,
   hash::{Hash,Hasher}
};

use book::{
   csv_utils::CsvWriter,
   file_utils::lines_from_file,
   json_utils::unquot,
   num_utils::mk_estimate
};

#[derive(Debug, Clone)]
pub struct Book {
   // e.g.: {"ask":"1.8020000000","base_currency":"LUNA",
   //        "base_volume":"899.7562950000","bid":"1.7890000000",
   //        "high":"1.8709996622","last_price":"1.7890005387",
   //        "low":"1.7609999772",
   //        "pool_id":"kujira1yg8930mj8...p0kur",
   //        "target_currency":"axlUSDC","target_volume":"1647.8921550000",
   //        "ticker_id":"LUNA_axlUSDC"},

   base: String,
   target: String,
   pool_id: String,
   pub vol_24h: f32
}

#[derive(Deserialize)]
struct Books {
   #[serde(rename(deserialize="tickers"))]
   books: Vec<Book>
}

pub fn load_books(filename: &str) -> HashSet<Book> {
   let file = lines_from_file(&filename).join(" ");
   parse_books(&file)
}

pub fn parse_books(str: &str) -> HashSet<Book> {
   let books: Books = from_str(str).expect("booked!");
   books.books.into_iter().collect()
}

impl<'de> Deserialize<'de> for Book {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
          where D: Deserializer<'de> {
      let json: Value = Value::deserialize(deserializer)?;
      let base = unquot(&json, "base_currency");
      let target = unquot(&json, "target_currency");
      let pool_id = unquot(&json, "pool_id");
      let ask1 = unquot(&json, "ask");
      let ask: f32 = ask1.parse().expect("ask");
      let vol_raw2 = unquot(&json, "base_volume");
      let vol_raw: f32 = vol_raw2.parse().expect("24h vol");
      let vol_24h = vol_raw * ask;
      Ok(Book { base, target, pool_id, vol_24h })
   }
}

impl Hash for Book {
   fn hash<H: Hasher>(&self, state: &mut H) {
      self.base.hash(state);
      self.target.hash(state);
      self.pool_id.hash(state);
   }
}

impl PartialEq for Book {
   fn eq(&self, other: &Self) -> bool {
      self.base == other.base
         && self.target == other.target
         && self.pool_id == other.pool_id
   }
}

impl Eq for Book {}

impl CsvWriter for Book {
   fn as_csv(&self) -> String {
      format!("{},{}", ticker(self), estimate(self))
   }
}

pub fn fetch_books(fin: &HashSet<Book>, token: &str) -> HashSet<Book> {
   book_fetcher(|b| b.base == token || b.target == token, fin)
}

pub fn fetch_books_by_vol(fin: &HashSet<Book>, vol: f32) -> HashSet<Book> {
   book_fetcher(|b| b.vol_24h > vol, fin)
}

pub fn book_fetcher(f: impl Fn(&Book) -> bool, fin: &HashSet<Book>)
   -> HashSet<Book> {
   let mut ans = HashSet::new();
   for b in fin {
      if f(b) { ans.insert(b.clone()); }
   }
   ans
}

pub fn ticker(b: &Book) -> String {
   format!("{}/{}", b.base, b.target)
}

pub fn url(b: &Book) -> String {
   format!("https://fin.kujira.app/trade/{}", b.pool_id)
}

pub fn estimate(b: &Book) -> String {
   format!("${}", mk_estimate(b.vol_24h))
}

pub fn count(books: &HashSet<Book>, token: &str) -> usize {
   let ans = fetch_books(&books, token).len();
   println!("There are {ans} {token} books");
   ans
}
