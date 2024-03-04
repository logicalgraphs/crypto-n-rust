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
   num_utils::mk_estimate,
   utils::pred
};

use crate::types::{
   marketplace::{OrderBook,mk_orderbook},
   usd::{USD,mk_usd,no_monay}
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
   pub last: f32
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
      let last: f32 = lask1.parse().expect("last_price");
      let vol_raw2 = unquot(&json, "base_volume");
      let vol_raw: f32 = vol_raw2.parse().expect("24h vol");
      let vol_24h = vol_raw * last;
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
   fn ncols(&self) -> usize { 2 }
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

pub fn book_orderbook(prices: &HashMap<String, USD>)
    -> impl Fn(&Book) -> OrderBook + '_ {
   |b| {
      let base = &b.base;
      let err_msg = format!("Calamity! No price for {base}!");
      let price = prices.get(base).expect(&err_msg);
      let ratio = b.last;
      mk_orderbook(base, &b.target, ratio, &price)
   }
}

pub fn prices(books: &HashSet<Book>) -> HashMap<String, USD> {
   let mut prices: HashMap<String, USD> = HashMap::new();
   let usdcs = fetch_books(books, "axlUSDC");
   let usk = usk_price(&usdcs);
   prices.insert("USK".to_string(), usk);
   prices.insert("axlUSDC".to_string(), mk_usd(1.0));
   let mut other_books: HashSet<Book> = books.clone();

   // we insert the USDC order book prices, because last_price==ratio==quote
   // for the USDC order books, at any rate.

   for b in usdcs {
      mb_insert(&mut prices, &b);
      other_books.remove(&b);
   }

   // we insert the USK order books for those tokens that have only an USK
   // counterpart.

   let usks = fetch_books(&other_books, "USK");

   for b in usks {
      mb_insert_usk(&mut prices, &b, &usk);
      other_books.remove(&b);
   }

   // then we load all the remaining prices. Now, last_price === ratio

   for b in other_books {
      mb_insert_price(&mut prices, &b);
   }
   prices
}

// a new take on prices

// FRIST! I load all axlUSDC prices (where prices are over $0.00)
// then I overlay with USDC-prices and USK-prices (after converting USK
// to axlUSDC-equivalent).

// THEN I take the remaining order books and ratio their prices from base
// price. Maybe I could just oracle everything, instead?

/*
pub fn prices_2(books: &HashSet<Book>) -> HashMap<String, USD> {
   let (axls, b1) = part(&books, "axlUSDC");
   let stables1: HashSet<&Book> =
      books.iter().filter(|b| b.base == "axlUSDC").collect();

   let mut stables2: HashSet<Book> = HashSet::new();
      // books.into_iter().filter(|b| b.base == "axlUSDC").map(Clone::clone).collect();  ... why is this infinite loop?

   for b in books {
      if b.base == "axlUSDC" { stables2.insert(b.clone()); }
   }

   // okay, now I have the axlUSDC order books. That means I can get the
   // USK and USDC prices, relative to axlUSDC, which I am using as peg,
   // until I am convinced otherwise.

   // now let's add the prices, layer-by-layer, starting with axlUSDC-prices.

   fn mb_book(factor: &USD) -> impl Fn(&Book) -> Option<(String, USD)> + '_ {
      | b | {
         pred(b.last > 0.0 && b.vol_24h > 0.0,
              (b.base.clone(), mk_usd(b.last * factor.amount)))
      }
   }
   fn mk_books(dollah: &USD, src: &HashSet<Book>) -> HashMap<String, USD> {
      src.into_iter().filter_map(mb_book(dollah)).collect()
   }
   let axl_usdc = mk_usd(1.0);
   let paxl = mk_books(&axl_usdc, &axls);

   print_books("axlUSDC", &paxl);      // FIXME! remove
   let stables = mk_books(&axl_usdc, &stables2);
   print_books("stables", &stables);

   let usk = stable_price(&stables1, "USK");
   let usdc = stable_price(&stables1, "USDC");

// we must add usk, usdc, and axlUSDC to the HashMap, ... OR ELSE! :<

   fn print_stable(n: &str, s: &USD) {
      println!("{n}: {s}");
   }
   print_stable("USK", &usk);
   print_stable("USDC", &usdc);

   let (usdcs, _b2) = part(&b1, "USDC");
   let qusdc = mk_books(&usdc, &usdcs).into_iter().chain(paxl).collect();
   // print_books("USDC", &qusdc);
}
*/

pub fn prices_3(books: &HashSet<Book>) -> HashMap<String, USD> {
   let (stables, unstables) = stable_books(books);
   let (axls, others) = books_for("axlUSDC", (&stables, &unstables));
   let (usdcs, tail) = books_for("USDC", (&stables, &others));
   let (usks, _rest) = books_for("USK", (&stables, &tail));
   let prices =
      usdcs.into_iter().chain(usks).chain(axls).chain(stables).collect();

   // now the _rest's are fun!!! These are the order books that don't have a
   // stable target, SO! we need to use the prices HashMap to find the price
   // of the target to compute the price of the base.
   prices
}

type VPair<T> = (HashSet<T>, HashSet<T>);
type BookBooks = (HashMap<String, USD>, HashSet<Book>);
type BookBooksRef<'a> = (&'a HashMap<String, USD>, &'a HashSet<Book>);

fn part(f: impl Fn(&Book) -> &str, v: &HashSet<Book>, p: &str) -> VPair<Book> {
   // why am I writing:
   // v.into_iter().partition(|b| b.target == p)
   // f'n copy-semantics and Rust, stg.
   let mut left = HashSet::new();
   let mut right = HashSet::new();
   for b in v {
      (if f(b) == p { &mut left } else { &mut right }).insert(b.clone());
   }
   (left, right)
}

fn books_for(stable: &str, (stables, books): BookBooksRef) -> BookBooks {
   let (mines, yourses) = part(move |b: &Book| &b.target, books, stable);

   fn mb_book(factor: &USD) -> impl Fn(&Book) -> Option<(String, USD)> + '_ {
      | b | {
         pred(b.last > 0.0 && b.vol_24h > 0.0,
              (b.base.clone(), mk_usd(b.last * factor.amount)))
      }
   }
   fn mk_books(dollah: &USD, src: &HashSet<Book>) -> HashMap<String, USD> {
      src.into_iter().filter_map(mb_book(dollah)).collect()
   }
   let quote = stables.get(stable).unwrap();
   (mk_books(quote, &mines), yourses)
}

fn stable_books(books: &HashSet<Book>) -> BookBooks {
   let (stables, unstables) = part(|b: &Book| &b.base, books, "axlUSDC");
   let mut books = HashMap::new();
   for s in stables {
      books.insert(s.target.clone(), compute_stable_price(&s));
   }
   books.insert("axlUSDC".to_string(), mk_usd(1.0));  // just how I rollz, yo!
   (books, unstables)
}

/*
fn print_books(title: &str, books: &HashMap<String, USD>) {
   println!("\n{title} books");
   books.into_iter().for_each(|(a,b)| println!("{a}: {b}"));
   println!("");
}
*/

fn usk_price(usdcs: &HashSet<Book>) -> USD {
   // let prim: HashSet<&Book> = usdcs.into_iter().collect();
   // stable_price(&prim, "USK")
   stable_price(usdcs, "USK")
}

fn stable_price(axlusdcs: &HashSet<Book>, stable: &str) -> USD {
   let stables = axlusdcs.iter().find(|b| b.target == stable);
   if let Some(s) = stables {
      compute_stable_price(&s)
   } else {
      panic!("Could not find {stable} price in order books!")
   }
}

fn compute_stable_price(b: &Book) -> USD { mk_usd(1.0 / b.last) }

fn mb_insert(hm: &mut HashMap<String, USD>, b: &Book) {
   mb_insert_f(hm, b, |x| mk_usd(x.last))
}

fn mb_insert_usk(hm: &mut HashMap<String, USD>, b: &Book, usk: &USD) {
   mb_insert_f(hm, b, |x| mk_usd(x.last * usk.amount))
}

fn mb_insert_price(hm: &mut HashMap<String, USD>, b: &Book) {
   let h2 = hm.clone();
   mb_insert_f(hm, b, |x| price(x, &h2))
}

fn mb_insert_f(hm: &mut HashMap<String, USD>, b: &Book,
               f: impl Fn(&Book) -> USD) {
   let key = &b.base;
   if !hm.contains_key(key) {
      let zot = if b.vol_24h > 100.0 { f(b) } else { no_monay() };
      hm.insert(key.clone(), zot);
   }
}

fn price(b: &Book, hm: &HashMap<String, USD>) -> USD {
   if let Some(targe) = hm.get(&b.target) {
      mk_usd(targe.amount * b.last)
   } else {
      panic!("Could not find {} price!", b.target)
   }
}
