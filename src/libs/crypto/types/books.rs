// extracts current market data from https://api.kujira.app/api/coingecko/tickers

extern crate serde;

use serde::{Deserialize,Deserializer};
use serde_json::{Value, from_str};

use std::{
   collections::{HashMap,HashSet},
   hash::{Hash,Hasher}
};

use book::{
   csv_utils::CsvWriter,
   file_utils::lines_from_file,
   json_utils::unquot,
   num_utils::mk_estimate
};

use crate::types::{
   marketplace::{OrderBook,mk_orderbook},
   usd::{USD,mk_usd}
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
   pub vol_24h: f32,
   pub last: USD
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
      let lask1 = unquot(&json, "last_price");
      let lask: f32 = lask1.parse().expect("last_price");
      let vol_raw2 = unquot(&json, "base_volume");
      let vol_raw: f32 = vol_raw2.parse().expect("24h vol");
      let vol_24h = vol_raw * lask;
      let last = mk_usd(lask);
      Ok(Book { base, target, pool_id, vol_24h, last })
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

pub fn book_orderbook(target_prices: &HashMap<String, USD>)
   -> impl Fn(&Book) -> OrderBook + '_ {
   |b| {
      let sold = &b.target;
      let err_msg = format!("Calamity! No price for {sold}!");
      let targe = target_prices.get(sold).expect(&err_msg);
      let ratio = b.last.amount / targe.amount;
      mk_orderbook(&b.base, &b.target, ratio, &b.last)
   }
}

pub fn prices(books: &HashSet<Book>) -> HashMap<String, USD> {
   let mut prices: HashMap<String, USD> = HashMap::new();
   let usdcs = fetch_books(books, "axlUSDC");
   let usk = usk_price(&usdcs);
   prices.insert("USK".to_string(), usk);
   prices.insert("axlUSDC".to_string(), mk_usd(1.0));

   // we give precedence to the USDC order book prices.
   for b in usdcs {
      mb_insert(&mut prices, &b);
   }

   // then we load all the prices
   for b in books {
      mb_insert(&mut prices, &b);
   }
   prices
}

fn usk_price(usdcs: &HashSet<Book>) -> USD {
   let usks = usdcs.into_iter().find(|b| b.target == "USK");
   if let Some(u) = usks {
      mk_usd(1.0/ u.last.amount)
   } else {
      panic!("Could not find USK price in order books!")
   }
}

fn mb_insert(hm: &mut HashMap<String, USD>, b: &Book) {
   let key = &b.base;
   if !hm.contains_key(key) {
      hm.insert(key.clone(), b.last);
   }
}
