use std::{
   path::Path
};

use book::{
   utils::get_args,
   file_utils::lines_from_file,
   csv_utils::print_csv,
   list_utils::{head,tail}
};

use crypto::types::{
   trades::read_csv_swap
};

fn usage() {
   println!("\n./trader <trades CSV file>");
   println!("\n\tprints the trades that occurred, ... and stuff.");
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
   let lines = tail(lines_from_file(file));
   println!("date,from,to");
   lines.iter().for_each(print_trade);
}

fn print_trade(line: &String) {
   match read_csv_swap(line) {
      Ok(trade) => { print_csv(&trade); },
      Err(msg) =>  { println!("ERROR: {}", msg) }
   }
}
