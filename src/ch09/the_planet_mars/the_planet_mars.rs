extern crate reqwest; // 0.9.18

extern crate serde;

use serde::{Deserialize,Deserializer};
use serde_json::{Value, from_str};

use std::{
   collections::HashSet,
   hash::{Hash,Hasher},
   io::Read
};

use book::{
   csv_utils::CsvWriter,
   html_utils::a,
   json_utils::unquot,
   num_utils::mk_estimate,
   report_utils::{Mode, print_footer, print_top_of},
   utils::get_args
};

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
   pool_id: String,
   vol_24h: f32
}

struct LinkedBook { book: Book }

fn mk_linked(b: &Book) -> LinkedBook { LinkedBook { book: b.clone() } }

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
      format!("{}/{},${}", self.base, self.target, mk_estimate(self.vol_24h))
   }
}

impl CsvWriter for LinkedBook {
   fn as_csv(&self) -> String {
      let book = &self.book;
      let lnk = a(&url(&self.book), &format!("{}/{}", book.base, book.target));
      let vol = mk_estimate(book.vol_24h);
      format!("{lnk},${vol}")
   }
}

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

fn usage() {
   println!("./top_order_books <date>\n");
   println!("\tGives the top order books by volume");
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
   let args = get_args();
   if let Some(date) = args.first() {
      let mut res =
         reqwest::get("https://api.kujira.app/api/coingecko/tickers")?;
      let mut body = String::new();
      res.read_to_string(&mut body)?;
      reportage(&date, &body);
   } else {
      usage();
   }
   Ok(())
}

fn reportage(date: &str, body: &str) {
   let books = parse_books(&body);
   println!("I got {} books", books.len());
   // let books5: Vec<Book> = books.clone().into_iter().take(5).collect();
   // println!("\nThe first 5 of which are:\n{books5:?}\n");
   count(&books, "axlUSDC");
   count(&books, "USK");
   let mars = fetch_books(&books, "MARS");
   println!("The MARS books URLs are:");
   for m in mars {
      println!("{}: {}", ticker_id(&m), url(&m));
   }
   let mut tops: Vec<Book> = books.into_iter().collect();
   tops.sort_by(|a, b| b.vol_24h.partial_cmp(&a.vol_24h).unwrap());
   let topus: Vec<Book> =
      tops.into_iter().take_while(|b| b.vol_24h > 1000.0).collect();
   print_txt(&topus, date);
   print_html(&topus, date);
}

fn print_txt(tops: &Vec<Book>, date: &str) {
   printer(tops, date, &Mode::TEXT);
}

fn foot(mode: &Mode) {
   print_footer(mode, "src/ch09/top_order_books", "top_order_books");
}

fn print_html(tops: &Vec<Book>, date: &str) {
   let linkies: Vec<LinkedBook> = tops.into_iter().map(mk_linked).collect();
   printer(&linkies, date, &Mode::HTML);
}

fn printer<T: CsvWriter>(tops: &Vec<T>, date: &str, mode: &Mode) {
   print_top_of("FIN order books by volume", date, tops, mode);
   foot(mode);
}
