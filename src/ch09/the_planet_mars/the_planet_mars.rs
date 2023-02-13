extern crate reqwest; // 0.9.18

extern crate serde;

use serde::{Deserialize,Deserializer};
use serde_json::{Value, from_str};

use std::{
   collections::HashSet,
   hash::{Hash,Hasher},
   io::Read
};

use book::json_utils::unquot;

// The skeleton upon which this get-fetch example is based is:
// https://stackoverflow.com/questions/43222429/how-do-you-make-a-get-request-in-rust#:~:text=Sending%20a%20GET%20request%20is,send().unwrap()%3B%20assert_eq!

#[derive(Debug, Clone)]
struct Book {
   // e.g.: {"ask":"1.8020000000","base_currency":"LUNA",
   //        "base_volume":"899.7562950000","bid":"1.7890000000",
   //        "high":"1.8709996622","last_price":"1.7890005387",
   //        "low":"1.7609999772",
   //        "pool_id":"kujira1yg8930mj8...p0kur",
   //        "target_currency":"axlUSDC","target_volume":"1647.8921550000",
   //        "ticker_id":"LUNA_axlUSDC"},

// not this anymore:
   //        "pool_id":"kujira1nm3yktzc...v849dd3ulaygw75mqqxvtnck",
   //        "target":"USK",
   //        "ticker_id":"STARS_USK"},
   base: String,
   target: String,
   pool_id: String
}

#[derive(Deserialize)]
struct Books {
   #[serde(rename(deserialize="tickers"))]
   books: Vec<Book>
}

fn parse_books(str: &str) -> HashSet<Book> {
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
      Ok(Book { base, target, pool_id })
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

fn fetch_books(fin: &HashSet<Book>, token: &str) -> HashSet<Book> {
   let mut ans = HashSet::new();
   for b in fin {
      if b.base == token || b.target == token { ans.insert(b.clone()); }
   }
   ans
}

fn ticker_id(b: &Book) -> String {
   format!("{}_{}", b.base, b.target)
}

fn url(b: &Book) -> String {
   format!("https://fin.kujira.app/trade/{}", b.pool_id)
}

fn count(books: &HashSet<Book>, token: &str) -> usize {
   let ans = fetch_books(&books, token).len();
   println!("There are {ans} {token} books");
   ans
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
   let mut res =
      reqwest::get("https://api.kujira.app/api/coingecko/tickers")?;
   let mut body = String::new();
   res.read_to_string(&mut body)?;

   let books = parse_books(&body);
   println!("I got {} books", books.len());
   let books5: Vec<Book> = books.clone().into_iter().take(5).collect();
   println!("\nThe first 5 of which are:\n{books5:?}\n");
   count(&books, "axlUSDC");
   count(&books, "USK");
   let mars = fetch_books(&books, "MARS");
   println!("The MARS books URLs are:");
   for m in mars {
      println!("{}: {}", ticker_id(&m), url(&m));
   }
   Ok(())
}
