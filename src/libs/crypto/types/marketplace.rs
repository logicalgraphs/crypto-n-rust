// The marketplace is a set of orderbooks

// I think that a hash-set works here, as we have a complex key, that key being:

// KUJI-axlUSDC, where we have to locate the order books that have sought-after
// asset on either side of the pair, but then we have to know that on one side
// it's a buy-conversation, but on the other side it's a sell-conversation.

// first things first: ingest.

// we have this:

use std::{
   fmt,
   collections::HashSet,
   hash::{Hash,Hasher}
};

use book::list_utils::head;

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

impl fmt::Display for OrderBook {
   fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
      write!(formatter, "{} / {} {:.3} {}", self.buy_side, self.sell_side,
             self.ratio, self.price)
   }
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

pub fn mk_orderbook(buy_side: String, sell_side: String, ratio: f32, price: USD)
   -> OrderBook {
   OrderBook { buy_side, sell_side, ratio, price }
}

// ----- scanners/parsers ---------------------------------------------------

pub fn parse_orderbook(buy: &str, sell: &str, rat: &str, pric: &str)
   -> Result<OrderBook, String> {
   let ratio: f32 = rat.parse().expect("ratio");
   let pric1: f32 = pric.parse().expect("price");
   let price: USD = mk_usd(pric1);
   let buy_side = buy.to_string();
   let sell_side = sell.to_string();
   Ok(mk_orderbook(buy_side, sell_side, ratio, price))
}

pub fn scan_orderbook(lines: Vec<String>)
   -> (Result<OrderBook, String>, Vec<String>) {
   let (order, rest) = lines.split_at(6);
   (if let [buy, sell, rat, _sign, pric, _change] = order {
      parse_orderbook(buy, sell, rat, pric)
   } else {
      match head(order.to_vec()) {
         Some(buy) => Err("Can't parse pair starting with: ".to_owned() + &buy),
         None      => Err("Panik at ze Disco!".to_string())
      }
   }, rest.to_vec())
}

pub fn parse_lines(n: u32, books: &mut HashSet<OrderBook>, lines: Vec<String>) {
   parse_lines_debug(n, books, lines, false);
}

pub fn parse_lines_debug(n: u32, books: &mut HashSet<OrderBook>,
                         lines: Vec<String>, debug: bool) {
   if debug { println!("Processing order book {}", n); }
   let (mb_order, rest) = scan_orderbook(lines);
   match mb_order {
      Ok(book) => {
         if debug { println!("Processed {:?}", book); }
         books.insert(book);
      },
      Err(msg) => println!("{}", msg)
   };
   if rest.len() > 0 {
      parse_lines(n + 1, books, rest);
   }
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
