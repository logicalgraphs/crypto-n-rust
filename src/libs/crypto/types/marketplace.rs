// The marketplace is a set of orderbooks

// I think that a hash-set works here, as we have a complex key, that key being:

// KUJI-axlUSDC, where we have to locate the order books that have sought-after
// asset on either side of the pair, but then we have to know that on one side
// it's a buy-conversation, but on the other side it's a sell-conversation.

// first things first: ingest.

// we have this:

use std::{
   collections::{HashSet,HashMap},
   fmt,
   hash::{Hash,Hasher}
};

use book::{
   csv_utils::{CsvWriter,print_csv},
   err_utils::ErrStr,
   file_utils::lines_from_file,
   list_utils::head
};

use crate::types::{
   assets::Asset,
   usd::{USD, mk_usd}
};

/*
date: 2022-10-18

Pair	Last Price	Change
KUJI
axlUSDC
1.051
$
1.050
-0.90%
... etc
*/

#[derive(Debug, Clone)]
pub struct OrderBook {
   buy_side: String,
   sell_side: String,
   pub ratio: f32,
   price: USD
}

// ----- impl -------------------------------------------------------

impl CsvWriter for OrderBook {
   fn as_csv(&self) -> String { csv(self) }
   fn ncols(&self) -> usize { 5 }
}

impl fmt::Display for OrderBook {
   fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
      write!(formatter, "{} / {} {:.3} {}", self.buy_side, self.sell_side,
             self.ratio, self.price)
   }
}

// the simple representation:

pub fn orderbook(o: &OrderBook) -> String {
   format!("{}/{}", o.buy_side, o.sell_side)
}

impl Hash for OrderBook {
   fn hash<H: Hasher>(&self, state: &mut H) {
      self.buy_side.hash(state);
      self.sell_side.hash(state);
      self.price.hash(state);
   }
}

impl PartialEq for OrderBook {
   fn eq(&self, other: &Self) -> bool {
      self.buy_side == other.buy_side
      && self.sell_side == other.sell_side
   }
}

impl Eq for OrderBook {}

// ----- constructors -------------------------------------------------------

pub fn mk_orderbook(buy: &str, sell: &str, ratio: f32, pric: &USD)
   -> OrderBook {
   let buy_side = buy.to_string();
   let sell_side = sell.to_string();
   let price = mk_usd(pric.amount);
   OrderBook { buy_side, sell_side, ratio, price }
}

// ----- Sythetic order books ---------------------------------------------

/*
Sythetic order books are order books, that are not order books.

Examples: ATOM/OSMO is the Achilles Heel of Kujira, so I use the ATOM/OSMO
swap on the Osmosis Zone. Same for ampLUNA/LUNA on Terra, same for 
stATOM/ATOM on Stride.

A synthetic order book is constructed from the existing prices of Kujira's
order book prices, but is manually provided the ratio (from external
trade results).

So, synthetic order books walk and talk just like order books, they just
so happen to be loaded in differently and can be treated differently when
discovering arbitration paths.
*/

pub fn read_synthetic_order_books(file: &str, quotes: &HashMap<String, USD>)
      -> ErrStr<HashSet<OrderBook>> {
   let lines = lines_from_file(file)?;
   let (_header, rows) = lines.split_at(4);
   let mut pairs = HashSet::new();
   for row in rows {
      let tsv: Vec<&str> = row.split('\t').collect();
      if let [buy, sell, rat] = tsv.as_slice() {
        let ratio = rat.parse().expect(&format!("Synthetic ratio {rat}"));
        if let Some(price) = quotes.get(&buy.to_string()) {
           pairs.insert(mk_orderbook(buy, sell, ratio, price));
        }
      } else {
        println!("Could not process line {row}");
      }
   }
   Ok(pairs)
}

pub fn merge_synthetics(markets: &mut HashSet<OrderBook>,
                        quotes: &HashMap<String, USD>,
			synthetics: &str) -> ErrStr<()> {
   let synths = read_synthetic_order_books(&synthetics, &quotes)?;
   for s in synths {
      markets.insert(s.clone());
   }
   Ok(())
}

// ----- Access -------------------------------------------------------

pub fn fetch_orderbooks(markets: &HashSet<OrderBook>, token: &String)
   -> HashSet<OrderBook> {
   let mut ans = HashSet::new();
   for o in markets.iter() {
      if &o.buy_side == token || &o.sell_side == token {
         ans.insert(o.clone());
      }
   }
   ans
}

pub fn fetch_orderbooks_for(markets: &HashSet<OrderBook>, a: &Asset)
   -> HashSet<OrderBook> {
   fetch_orderbooks(markets, &a.token)
}

pub fn dual_asset(o: &OrderBook, a: &Asset) -> String {
   (if o.buy_side == a.token { &o.sell_side } else { &o.buy_side }).clone()
}

// ----- Bases -------------------------------------------------------

// Okay, the buy-side assets. There are two cases:

// 1. multiple: in this case, there is always an axlUSDC-dual
// 2. uno: then there's either axlUSDC- or USK-dual, but in either pairing,
//    there is an axlUSDC pricing for this asset, so no conversion is necessary
//    for the axlUSDC-price

// so the only complication is to reduce the n-tuple to a 1-tuple-axlUSDC-dual
// then map those to name-quote pairs

// this reduction can be simplified to this transformation:

// hashset<orderbooks> -> hashmap<buy, [(sell, quote)]>
// then filter on axlUSDC where right-side length > 1
// THEN < buy, snd(head(filtered))> gets you the bases for $axlUSDC prices

fn fetch_buys(market: &HashSet<OrderBook>)
   -> HashMap<String, HashMap<String, USD>> {
   let mut ans = HashMap::<String, HashMap<String, USD>>::new();
   for o in market.iter() {
      let key = &o.buy_side;
      ans.insert(key.clone(), insert_then(ans.get(key), &o));
   }
   ans
}

// addresses the 'problem' that insert doesn't return the modified HashMap

fn insert_then(m: Option<&HashMap<String, USD>>, o: &OrderBook)
   -> HashMap<String, USD> {
   let mut hash = match m {
      None => HashMap::new(),
      Some(h) => h.clone()
   };
   hash.insert(o.sell_side.clone(), o.price);
   hash
}

// this assumes that if there are multiple sales prices, one of this is
// axlUSDC. This is no longer the case with LP KUJI-ATOM.

// So, a workaround from the assumption, then? Joy.

fn extract_price(k: &str, sells: &HashMap<String, USD>) -> USD {
   if k == "axlUSDC" {
      if let Some(ans) = sells.values().collect::<Vec<_>>().first() {
         mk_usd(1.0 / ans.amount)
      } else { panic!("Cannot extract USK from axlUSDC order book!") }
   } else {
      extract_price1(sells)
   }
}

fn extract_price1(sells: &HashMap<String, USD>) -> USD {
   let hashes = if sells.len() > 1 {
      let works_mostly = filter_vals(|key| key == "axlUSDC", sells);
      if works_mostly.len() == 0 {
         let mut ans = sells.clone();
         ans.retain(|_, v| v.amount > 0.0);
         ans
      } else {
         works_mostly
      }
   } else { sells.clone() };
   let prices = hashes.values().collect();
   match head(&prices) {
      None => mk_usd(0.0),
      Some(dollah) => dollah.clone()
   }
}

fn filter_vals<V: Clone>(f: impl Fn(&String) -> bool, m: &HashMap<String, V>)
   -> HashMap<String, V> {
   let mut ans: HashMap<String, V> = HashMap::new();
   for (k, v) in m.iter() {
      if f(k) { ans.insert(k.clone(), v.clone()); }
   }
   ans
}

pub fn prices(market: &HashSet<OrderBook>) -> HashMap<String, USD> {
   let buys = fetch_buys(market);
   let mut ans: HashMap<String, USD> = HashMap::new();
   for (k, v) in buys.iter() {
      ans.insert(k.clone(), extract_price(&k, v));
   }
   ans
}

// once we have the prices (above), it's simple to convert to an USK-table:
// just look up the axlUSDC-price: that's on the axlUSDC/USK order book.

// but it's not that simple. Let's just consider the books that have USK

pub fn fetch_usk_books(market: &HashSet<OrderBook>) -> HashMap<String, f32> {
   let mut ans: HashMap<String, f32> = HashMap::new();
   for book in market {
      if book.sell_side == "USK".to_string() {
         ans.insert(book.buy_side.clone(), book.ratio);
      }
   }

   // we also need to add USK, itself, to this price-list.
   if let Some(usk) = ans.get("axlUSDC") {
      ans.insert("USK".to_string(), *usk);
   }
   ans
}

pub fn prices_usk(market: &HashSet<OrderBook>) -> HashMap<String, f32> {
   fetch_usk_books(market)
}

// ----- rekt-age -------------------------------------------------------

// Now, for recommendations, we want to sell our assets for a profit, so
// we consider the sell-side

pub fn fetch_sell_books(market: &HashSet<OrderBook>, a: &Asset)
   -> HashSet<OrderBook> {
   fetch_books(&|o: &OrderBook| o.sell_side.clone(), market, a)
}

pub fn fetch_buy_books(market: &HashSet<OrderBook>, a: &Asset)
   -> HashSet<OrderBook> {
   fetch_books(&|o: &OrderBook| o.buy_side.clone(), market, a)
}

fn fetch_books(f: &dyn Fn(&OrderBook) -> String, market: &HashSet<OrderBook>, 
               a: &Asset) -> HashSet<OrderBook> {
   let mut ans: HashSet<OrderBook> = HashSet::new();
   for o in market.iter() {
      if f(o) == a.token {
         ans.insert(o.clone());
      }
   }
   ans
}

// ----- output -------------------------------------------------------

fn csv(o: &OrderBook) -> String {
   let irat = inverse_ratio(o);
   format!("{},{},{},{},{}",o.buy_side,o.sell_side,o.ratio,irat,o.price)
}

pub fn print_marketplace(market: &HashSet<OrderBook>) {
   println!("buy,sell,ratio,inverse,price");
   market.iter().for_each(print_csv);
}

// ----- duals -------------------------------------------------------

pub fn inverse_ratio(o: &OrderBook) -> f32 {
   1.0 / o.ratio
}

// ---- a little hands-off reasoning about the marketplace -----------

pub fn ratio_for(os: &HashSet<OrderBook>, from: &String, to: &String) -> f32 {
   let mut ans = 0.0;
   let domain = fetch_orderbooks(os, from);
   let orderbooks = fetch_orderbooks(&domain, to); // should be 1 or 0

   for book in orderbooks {
      fn rat(book: &OrderBook) -> f32 { book.ratio }
      ans = if &book.buy_side == from { rat } else { inverse_ratio }(&book);
   }
   ans
}
