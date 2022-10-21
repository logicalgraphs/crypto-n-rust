use std::{
   collections::HashSet,
   path::Path
};

use book::{
   utils::get_args,
   file_utils::lines_from_file,
   list_utils::head
};

use crypto::types::marketplace::{OrderBook,scan_orderbook};

fn usage() {
   println!("\n./marketplace <marketplace LSV file>");
   println!("\n\tprints the marketplace tokens and prices");
}

fn main() {
   if let Some(filename) = head(get_args()) {
      println!("Processing {}", filename);
      parse_n_print(filename);
   } else {
      usage();
   }
}

fn parse_n_print(file: impl AsRef<Path>) {
   let lines = lines_from_file(file);
   let (_header, rows) = lines.split_at(3);
   let mut pairs = HashSet::new();
   parse_lines(1, &mut pairs, rows.to_vec());
   println!("From {} lines, I have {} order books", pairs.len(), lines.len());
}

fn parse_lines(n: u32, books: &mut HashSet<OrderBook>, lines: Vec<String>) {
   println!("Processing order book {}", n);
   let (mb_order, rest) = scan_orderbook(lines);
   match mb_order {
      Ok(book) => {
         println!("Processed {:?}", book);
         books.insert(book);
      },
      Err(msg) => println!("{}", msg)
   };
   if rest.len() > 0 {
      parse_lines(n + 1, books, rest);
   }
}
