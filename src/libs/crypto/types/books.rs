// extracts current market data from 
// https://api.kujira.app/api/coingecko/tickers

extern crate serde;

use serde::{Deserialize,Deserializer};
use serde_json::{Value, from_str};

use std::{
   clone::Clone,
   collections::{HashMap,HashSet},
   hash::{Hash,Hasher}
};

use book::{
   csv_utils::CsvWriter,
   json_utils::unquot,
   num_utils::mk_estimate,
   utils::pred
};

use crate::{
   rest_utils::read_market_json,
   types::{
      marketplace::{OrderBook,mk_orderbook},
      usd::{USD,mk_usd}
   }
};

#[derive(Debug, Clone)]
struct Book1 {
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
   base_vol: f32,
   target_vol: f32,
   last: f32
}

#[derive(Debug, Clone)]
pub struct Book {
   base: String,
   target: String,
   pool_id: String,
   base_vol: USD,
   target_vol: USD,
   last: f32
}

pub fn vol_24h(b: &Book) -> USD { b.base_vol + b.target_vol }

#[derive(Deserialize)]
struct BooksVec {
   #[serde(rename(deserialize="tickers"))]
   books: Vec<Book1>
}

pub type Prices = HashMap<String, USD>;
pub type Books = HashSet<Book>;
type Books1 = HashSet<Book1>;
pub type BookBooks = (Prices, Books);

fn raw_books() -> Books1 {
   let str = read_market_json().expect("Could not read FIN market data");
   let books: BooksVec = from_str(&str).expect("booked!");
   books.books.into_iter().collect()
}

// a ... 'little' function that transforms books of token-counts to books
// of USD-volumes. It's actually just a simple monadic-chain, at base.

fn books2books(p: &Prices, bs: &Books1) -> Books {
   fn price_tokens(p: &Prices) -> impl Fn(&Book1) -> Option<Book> + '_ {
      | b0 | {
         p.get(&b0.base).and_then(|b_price| {
            p.get(&b0.target).and_then(|t_price| {
               Some(Book { base: b0.base.clone(),
                           target: b0.target.clone(),
                           pool_id: b0.pool_id.clone(),
                           base_vol: mk_usd(b_price.amount * b0.base_vol),
                           target_vol: mk_usd(t_price.amount * b0.target_vol),
                           last: b0.last
                         })
            })
         })
      }
   }
   bs.into_iter().filter_map(price_tokens(p)).collect()
}

pub fn parse_books() -> BookBooks {
   let b0 = raw_books();
   let p = prices_from_books(&b0);
   let b = books2books(&p, &b0);
   (p, b)
}

impl<'de> Deserialize<'de> for Book1 {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
          where D: Deserializer<'de> {
      let json: Value = Value::deserialize(deserializer)?;
      let base = unquot(&json, "base_currency");
      let target = unquot(&json, "target_currency");
      let pool_id = unquot(&json, "pool_id");
      let lask1 = unquot(&json, "last_price");
      let last: f32 = lask1.parse().expect("last_price");
      let vol_raw2 = unquot(&json, "base_volume");
      let base_vol: f32 = vol_raw2.parse().expect("base vol");
      let vol_raw1 = unquot(&json, "target_volume");
      let target_vol: f32 = vol_raw1.parse().expect("target vol");
      Ok(Book1 { base, target, pool_id, base_vol, target_vol, last })
   }
}

impl Hash for Book1 {
   fn hash<H: Hasher>(&self, state: &mut H) {
      self.base.hash(state);
      self.target.hash(state);
      self.pool_id.hash(state);
   }
}

impl Hash for Book {
   fn hash<H: Hasher>(&self, state: &mut H) {
      self.base.hash(state);
      self.target.hash(state);
      self.pool_id.hash(state);
   }
}

impl PartialEq for Book1 {
   fn eq(&self, other: &Self) -> bool {
      self.base == other.base
         && self.target == other.target
         && self.pool_id == other.pool_id
   }
}

impl PartialEq for Book {
   fn eq(&self, other: &Self) -> bool {
      self.base == other.base
         && self.target == other.target
         && self.pool_id == other.pool_id
   }
}

impl Eq for Book1 {}
impl Eq for Book {}

impl CsvWriter for Book {
   fn as_csv(&self) -> String {
      format!("{},{},{},{},{},{}",
              ticker(self), estimate(self),
              self.base, self.base_vol, self.target, self.target_vol)
   }
   fn ncols(&self) -> usize { 6 }
}

pub fn fetch_books(fin: &Books, token: &str) -> Books {
   book_fetcher(move |b: &Book| b.base == token || b.target == token, fin)
}

pub fn fetch_books_by_vol(fin: &Books, vol: USD) -> Books {
   book_fetcher(move |b: &Book| vol_24h(b) > vol, fin)
}

fn book_fetcher(f: impl Fn(&Book) -> bool, fin: &Books) -> Books {
   let mut q = HashSet::new();
   for b in fin {
      if f(b) { q.insert(b.clone()); }
   }
   q
}

pub fn ticker(b: &Book) -> String {
   format!("{}/{}", b.base, b.target)
}

pub fn url(b: &Book) -> String {
   format!("https://fin.kujira.app/trade/{}", b.pool_id)
}

pub fn estimate(b: &Book) -> String {
   format!("${}", mk_estimate(vol_24h(b).amount))
}

pub fn count(books: &Books, token: &str) -> usize {
   let ans = fetch_books(&books, token).len();
   println!("There are {ans} {token} books");
   ans
}

pub fn book_orderbook(prices: &Prices) -> impl Fn(&Book) -> OrderBook + '_ {
   |b| {
      let base = &b.base;
      let err_msg = format!("Calamity! No price for {base}!");
      let price = prices.get(base).expect(&err_msg);
      let ratio = b.last;
      mk_orderbook(base, &b.target, ratio, &price)
   }
}

// a new take on prices

// FRIST! I load all axlUSDC prices (where prices are over $0.00)
// then I overlay with USDC-prices and USK-prices (after converting USK
// to axlUSDC-equivalent).

// THEN I take the remaining order books and ratio their prices from base
// price. Maybe I could just oracle everything, instead?

pub fn prices() -> Prices {
   let b0 = raw_books();
   prices_from_books(&b0)
}

fn prices_from_books(books: &Books1) -> Prices {
   let (stables, unstables) = stable_books(books);
   let (axls, others) = books_for("axlUSDC", (&stables, &unstables));
   let (usdcs, tail) = books_for("USDC", (&stables, &others));
   let (usks, rest) = books_for("USK", (&stables, &tail));
   let prices = usdcs.into_iter()
                     .chain(usks)
                     .chain(axls)
                     .chain(stables)
                     .collect();  // please note:
                                  // for HashMap, chain() is not associative.
                                  // This means the LAST map I chain is the
                                  // MOST IMPORTANT for prices.

   // now the rest are fun!!! These are the order books that don't have a
   // stable target, SO! we need to use the prices-HashMap to find the price
   // of the target to compute the price of the base.

   let baros: HashMap<String, USD> = rest.iter()
       .filter_map(barometric_board(&prices))
       .collect();
   baros.into_iter().chain(prices).collect()
}

type VPair<T> = (HashSet<T>, HashSet<T>);
type Book1Books = (Prices, HashSet<Book1>);
type Book1BooksRef<'a> = (&'a Prices, &'a Books1);

fn part(f: impl Fn(&Book1) -> &str, v: &Books1, p: &str) -> VPair<Book1> {

   // why am I writing: v.into_iter().partition(|b| b.target == p)
   // in long-form? f'n copy-semantics and Rust, stg.

   let mut left = HashSet::new();
   let mut right = HashSet::new();
   for b in v {
      (if f(b) == p { &mut left } else { &mut right }).insert(b.clone());
   }
   (left, right)
}

// only consider prices from books that have had trades today ... functionally!

fn mb_book(factor: &USD) -> impl Fn(&Book1) -> Option<(String, USD)> + '_ {
   | b | {
      pred(b.last > 0.0 && b.target_vol + b.base_vol > 0.0,
           (b.base.clone(), mk_usd(b.last * factor.amount)))
   }
}

fn books_for(stable: &str, (stables, books): Book1BooksRef) -> Book1Books {
   let (mines, yourses) = part(move |b: &Book1| &b.target, books, stable);

   fn mk_books(dollah: &USD, src: &Books1) -> Prices {
      src.into_iter().filter_map(mb_book(dollah)).collect()
   }
   let quote = stables.get(stable).unwrap();
   (mk_books(quote, &mines), yourses)
}

fn stable_books(books: &Books1) -> Book1Books {
   let (stables, unstables) = part(|b: &Book1| &b.base, books, "axlUSDC");
   let mut books = HashMap::new();
   for s in stables {
      books.insert(s.target.clone(), compute_stable_price(&s));
   }
   books.insert("axlUSDC".to_string(), mk_usd(1.0));  // just how I rollz, yo!
   (books, unstables)
}

// Here, we take the books that don't have a stable target, or so I think, then
// compute the prices for the bases to round out the token-prices-list.

fn barometric_board(prices: &Prices)
         -> impl Fn(&Book1) -> Option<(String, USD)> + '_ {
   fn mb_price(b: &Book1) -> impl Fn(&USD) -> Option<(String, USD)> + '_ {
      |price| { mb_book(price)(b) }
   }
   |book| prices.get(&book.target).and_then(mb_price(book))
}

fn compute_stable_price(b: &Book1) -> USD { mk_usd(1.0 / b.last) }

// Now that we have order-book-volumes-by-token and token-prices, we can
// compute order-book-volumes(-by-price) on the active order books, returning
// the active order books, only.


