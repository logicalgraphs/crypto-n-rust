use std::{
   collections::{HashMap,HashSet},
   hash::{Hash,Hasher}
};

use book::csv_utils::CsvWriter;

use crate::types::{
   pairs::{Dyad,mk_dyan,Tag,mk_tag,untag},
   usd::{USD,mk_usd}
};

#[derive(Debug, Clone)]
pub struct Book {
   base: String,
   target: String,
   pool_id: String,
   base_vol: USD,
   target_vol: USD,
   last: f32
}

// -- ick
pub type VPair = Tag<USD>;  // a token-price-pair
   
pub fn vols(b: &Book) -> (VPair, VPair) {
   (mk_tag((b.base, b.base_vol)), mk_tag((b.target, b.target_vol)))
}
   
pub fn vol_24h(b: &Book) -> USD { unpair(&vol_24h_pair(b)).1 }
   
pub fn vol_24h_pair(book: &Book) -> Dyad<USD> {
   let (b, t) = vols(book);
   let (bk, bv) = untag(&b);
   let (tg, tv) = untag(&b);
   mk_dyad((bk, tg), mk_usd((bv.amount + tv.amount) / 2.0))
}

// -- unick

pub fn ticker(b: &Book) -> String { format!("{}/{}", b.base, b.target) }
pub fn trades_token(t: &str) -> impl Fn(&Book) -> bool {
   |b: &Book| b.target == t || b.base == t
}

pub fn estimate(b: &Book) -> String {
   format!("${}", mk_estimate(vol_24h(b).amount))
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

pub fn count(books: &Books, token: &str) -> usize {
   let ans = fetch_books(&books, token).len();
   println!("There are {ans} {token} books");
   ans
}


pub type Books = HashSet<Book>;
pub type BookBooks = (Prices, Books);

pub type Price = Tag<USD>;
pub fn get_price(p: &Price) -> USD { untag(&p).1 }
pub fn mk_price(d: &str, p: USD) -> Price { mk_tag((d.to_string(), p)) }

pub type Prices = HashMap<String, Price>;

// ----- impls -------------------------------------------------------

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
   fn ncols(&self) -> usize { 6 }
}
