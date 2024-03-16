// extracts current market data from 
// https://api.kujira.app/api/coingecko/tickers

extern crate serde;

use serde::{Deserialize,Deserializer};
use serde_json::{Value, from_str};

use std::{
   collections::{HashMap,HashSet},
   hash::{Hash,Hasher}
};

use book::{
   csv_utils::CsvWriter,
   err_utils::ErrStr,
   file_utils::lines_from_file,
   json_utils::unquot,
   num_utils::mk_estimate,
   string_utils::parse_lines,
   utils::pred
};

use crate::{
   rest_utils::{read_market_json,read_aliases},
   types::{
      marketplace::{OrderBook,mk_orderbook},
      usd::{USD,mk_usd,no_monay}
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

pub type Books = HashSet<Book>;

// ----- Parsing -------------------------------------------------------

#[derive(Deserialize)]
struct BooksVec {
   #[serde(rename(deserialize="tickers"))]
   books: Vec<Book1>
}

pub type Prices = HashMap<String, USD>;
type Books1 = HashSet<Book1>;
pub type BookBooks = (Prices, Books);

fn raw_books() -> Books1 {
   let str = read_market_json().expect("Could not read FIN market data");
   let books: BooksVec = from_str(&str).expect("booked!");
   books.books.into_iter().collect()
}

/* 
A special case treating protocols-blockchains or tokens-blockchains
as 'order books' because they have the same structure: the end-game is
to structure these data as a Venn Diagram, or a graph, ... or both.

Or Voronoi? That will clean up some code.

Will I get burned by this semantic overloading?

Probably? Maybe?

But let's go with this for now and adapt as the end-game clarifies.

The structure of these 'books' are as follows, a TSV-file of the form:

blah-di-blah,some other stuff,date

protocol/token,blockchain,_invested,value $,other stuff,...

e.g.:

Blockaverse	portfolio	2024-03-15					
							
Token	Blockchain	invested	value	gain/loss	ROI	real token name	rôle
GMX	Arbitrum	$933.00	$682.45	-$250.55	-26.85%	GMX	
BTC	Cardano	$409.29	$867.55	$458.26	111.97%	iBTC	blue-chip
*/

pub fn load_books_from_file(filename: &str) -> ErrStr<Books> {
   let file = lines_from_file(filename);
   fn parser(line: String) -> ErrStr<Book> {
      let cols0: Vec<&str> = line.split("\t").collect();
      let cols: Vec<&str> = cols0.into_iter().take(4).collect();
      if let [tok, block, _, val] = cols.as_slice() {
         let u: USD =
            val.parse()
               .expect(&format!("Could not parse dollar value: '{val}'"));
         Ok(Book { base: tok.to_string(), target: block.to_string(),
                   base_vol: u.clone(), target_vol: u,
                   pool_id: "".to_string(), last: 0.0 })
      } else {
         Err(format!("Could not parse line: '{line}'"))
      }
   }
   let books: HashSet<Book> =
      parse_lines(parser, &file, Some(3))?.into_iter().collect();
   Ok(books)
}

// ----- Volumes -------------------------------------------------------

pub fn vol_24h(b: &Book) -> USD { vol_24h_pair(b).1 }

pub fn vol_24h_pair(b: &Book) -> ((String, String), USD) {
   let ((bk, bv), (tg, tv)) = vols(b);
   ((bk, tg), mk_usd((bv.amount + tv.amount) / 2.0))
}

pub type Volumes = HashMap<String, USD>;

pub fn volumes_by_token(bs: &Books) -> Volumes {
   let mut ans: Volumes = HashMap::new();
   for b in bs {
      let ((bk, bv), _) = vols(b);
      let bas = ans.entry(bk).or_insert(no_monay());
      *bas += bv;
   }
   for b in bs {
      let (_, (tg, tv)) = vols(b);
      let tar = ans.entry(tg).or_insert(no_monay());
      *tar += tv;
   }
   ans
}

// ----- Prices -------------------------------------------------------

type VPair = (String, USD);  // a token-price-pair

pub fn vols(b: &Book) -> (VPair, VPair) {
   ((b.base.clone(), b.base_vol.clone()),
    (b.target.clone(), b.target_vol.clone()))
}

// ----- Aliases -------------------------------------------------------

type Aliases = HashMap<String, String>;

fn alias(aliases: &Aliases, i: &String) -> String {
   aliases.get(i).unwrap_or(i).clone()
}

fn load_aliases(opt_url: &Option<String>) -> Aliases {
   let mut ans = HashMap::new();
   if let Some(url) = opt_url {
      let file = read_aliases(url).expect("Cannot read aliases file.");
      let all_lines: Vec<_> = file.split("\n").collect();
      let (_date, lines) = all_lines.split_at(3);

      for alias in lines {
         if let [id,name] = alias.split(",").collect::<Vec<_>>().as_slice() {
           ans.insert(id.to_string(), name.to_string());
         } else { println!("Unable to parse alias: '{alias}'") }
      }
   }
   ans
}

// a ... 'little' function that transforms books of token-counts to books
// of USD-volumes. It's actually just a simple monadic-chain, at base.

fn books2books(p: &Prices, bs: &Books1, aliases: &Aliases) -> Books {
   fn price_tokens<'a>(p: &'a Prices, a: &'a Aliases)
         -> impl Fn(&Book1) -> Option<Book> + 'a {
      | b0 | {
         let bas = alias(a, &b0.base);
         p.get(&bas).and_then(|b_price| {
            let tar = alias(a, &b0.target);
            p.get(&tar).and_then(|t_price| {
               Some(Book { base: bas,
                           target: tar,
                           pool_id: b0.pool_id.clone(),
                           base_vol: mk_usd(b_price.amount * b0.base_vol),
                           target_vol: mk_usd(t_price.amount * b0.target_vol),
                           last: b0.last
                         })
            })
         })
      }
   }
   bs.into_iter().filter_map(price_tokens(p, aliases)).collect()
}

pub fn parse_books(opt_aliases: Option<String>) -> BookBooks {
   let b0 = raw_books();
   let aliases = load_aliases(&opt_aliases);
   let p = prices_from_books(&b0, &aliases);
   let b = books2books(&p, &b0, &aliases);
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
      format!("{},{}", ticker(self), estimate(self))
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

pub fn prices(opt_aliases: Option<String>) -> Prices {
   let b0 = raw_books();
   let aliases = load_aliases(&opt_aliases);
   prices_from_books(&b0, &aliases)
}

fn prices_from_books(books: &Books1, aliases: &Aliases) -> Prices {
   let (stables, unstables) = stable_books(books, aliases);
   let (axls, others) =
       books_for("axlUSDC", (&stables, &unstables), aliases);
   let (usdcs, tail) = books_for("USDC", (&stables, &others), aliases);
   let (usks, rest) = books_for("USK", (&stables, &tail), aliases);
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

   let baros: Prices = rest.iter()
       .filter_map(barometric_board(&prices, aliases))
       .collect();
   baros.into_iter().chain(prices).collect()
}

type VHashes<T> = (HashSet<T>, HashSet<T>);
type Book1Books = (Prices, HashSet<Book1>);
type Book1BooksRef<'a> = (&'a Prices, &'a Books1);

fn part(f: impl Fn(&Book1) -> String, v: &Books1, p: &str) -> VHashes<Book1> {

   // why am I writing: v.into_iter().partition(|b| b.target == p)
   // in long-form? f'n copy-semantics and Rust, stg.

   let mut left = HashSet::new();
   let mut right = HashSet::new();
   for b in v {
      (if &f(b) == p { &mut left } else { &mut right }).insert(b.clone());
   }
   (left, right)
}

// only consider prices from books that have had trades today ... functionally!

fn mb_book<'a>(factor: &'a USD, a: &'a Aliases)
      -> impl Fn(&Book1) -> Option<(String, USD)> + 'a {
   | b | {
      pred(b.last > 0.0 && b.target_vol + b.base_vol > 0.0,
           (alias(a, &b.base), mk_usd(b.last * factor.amount)))
   }
}

fn books_for(stable: &str, (stables, books): Book1BooksRef, aliases: &Aliases)
      -> Book1Books {
   let (mines, yourses) =
      part(move |b: &Book1| alias(aliases, &b.target), books, stable);

   fn mk_books(dollah: &USD, src: &Books1, a: &Aliases) -> Prices {
      src.into_iter().filter_map(mb_book(dollah, a)).collect()
   }
   let quote = stables.get(stable).unwrap();
   (mk_books(quote, &mines, aliases), yourses)
}

fn stable_books(books: &Books1, a: &Aliases) -> Book1Books {
   let (stables, unstables) =
      part(|b: &Book1| alias(a, &b.base), books, "axlUSDC");
   let mut books = HashMap::new();
   for s in stables {
      books.insert(alias(a, &s.target), compute_stable_price(&s));
   }
   books.insert("axlUSDC".to_string(), mk_usd(1.0));  // just how I rollz, yo!
   (books, unstables)
}

// Here, we take the books that don't have a stable target, or so I think, then
// compute the prices for the bases to round out the token-prices-list.

fn barometric_board<'a>(prices: &'a Prices, a: &'a Aliases)
         -> impl Fn(&Book1) -> Option<(String, USD)> + 'a {
   fn mb_price<'b>(b: &'b Book1, a: &'b Aliases)
          -> impl Fn(&USD) -> Option<(String, USD)> + 'b {
      move |price| { mb_book(price, a)(b) }
   }
   |book| prices.get(&alias(a, &book.target)).and_then(mb_price(book, a))
}

fn compute_stable_price(b: &Book1) -> USD { mk_usd(1.0 / b.last) }

// Now that we have order-book-volumes-by-token and token-prices, we can
// compute order-book-volumes(-by-price) on the active order books, returning
// the active order books, only.
