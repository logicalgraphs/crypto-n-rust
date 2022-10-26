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
      let kuji_books = fetch_orderbooks(markets, "KUJI".to_string());
      println!("The KUJI order books are {:?}", kuji_books);
   } else {
      usage();
   }
}

fn parse_n_print(file: impl AsRef<Path>) -> HashSet<OrderBook> {
   let lines = lines_from_file(file);
   let (_header, rows) = lines.split_at(3);
   let mut pairs = HashSet::new();
   parse_lines(1, &mut pairs, rows.to_vec());
   println!("From {} lines, I have {} order books", lines.len(), pairs.len());
   pairs
}
