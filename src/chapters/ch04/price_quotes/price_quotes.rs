// Okay, let's read in the file and spit out symbols and price-quotes

use std::{
   clone::Clone,
   fmt
};

use book::{
   utils::{get_args,ht},
   file_utils::extract_date_and_body
};

struct Quote {
   symbol: String,
   price: String    // for now, ... we'll represent $USD sometime later mb
}

impl fmt::Display for Quote {
   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      write!(f, "{},{}", self.symbol, self.price)
   }
}

fn usage() {
   println!("Usage: ./price_quotes <filename>\n");
}

fn main() {
   if let [file] = get_args().as_slice() {
      let (date, all_lines) = extract_date_and_body(file);
      print_quotes(date, all_lines);
   } else { usage() }
}

fn extract_one_price_quote(lines: Vec<String>) -> (Option<Quote>, Vec<String>) {
   let (quote, rest) = lines.split_at(10);
   let mut quot = None;
   if let Some((sym, pric)) = first_two(quote.to_vec()) {
      quot = Some(Quote { symbol: sym, price: pric })
   }
   (quot, rest.to_vec())
}

fn first_two<T: Clone>(list: Vec<T>) -> Option<(T, T)> {
   let mut ans = None;
   if let (Some(x), rest) = ht(list) {
      if let (Some(y), _) = ht(rest) {
         ans = Some((x, y));
      }
   }
   ans
}

fn print_quotes(date: String, lines: Vec<String>) {
   if !lines.is_empty() {
      let (quot, rest) = extract_one_price_quote(lines);
      print_quote(&date, quot);
      print_quotes(date, rest);
   }
}

fn print_quote(date: &String, quot: Option<Quote>) {
   if let Some(qu) = quot {
      println!("{},{}", *date, qu);
   }
}
