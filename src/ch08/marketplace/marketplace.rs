use std::{
   collections::HashSet,
   path::Path
};

use book::{
   utils::get_args,
   file_utils::lines_from_file,
   list_utils::head
};

use crypto::types::marketplace::{OrderBook,parse_lines,fetch_orderbooks};

fn usage() {
   println!("\n./marketplace <marketplace LSV file>");
   println!("\n\tprints the marketplace tokens and prices");
}

fn main() {
   if let Some(filename) = head(get_args()) {
      println!("Processing {}", filename);
      let markets = parse_n_print(filename);
      let atom_books = fetch_orderbooks(&markets, &"ATOM".to_string());
      println!("The ATOM order books are:");
      for o in atom_books.iter() {
         println!("\t{}", o);
      }
   } else {
      usage();
   }
}

fn parse_n_print(file: impl AsRef<Path>) -> HashSet<OrderBook> {
   let lines = lines_from_file(file);
   let (_header, rows) = lines.split_at(3);
   let mut pairs = HashSet::new();
   parse_lines(&mut pairs, rows.to_vec());
   println!("From {} lines, I have {} order books", lines.len(), pairs.len());
   pairs
}
