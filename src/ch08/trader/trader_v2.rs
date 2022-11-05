// trail run with ./trader_v2 data/mini_me.csv

use std::{
   collections::HashSet,
   path::Path
};

use book::{
   utils::get_args,
   file_utils::lines_from_file,
   list_utils::{head,tail,ht}
};

use crypto::types::{
   assets::{Asset, print_assets},
   trades::{read_csv_swap,swap_d}
};

fn usage() {
   println!("\n./trader <trades CSV file>");
   println!("\n\tprints the trades that occurred, ... and stuff.");
}

fn main() {
   if let Some(filename) = head(get_args()) {
      println!("Processing {}", filename);
      parse_n_print(filename);
      println!("Finito!");  // a little Italian flourish at the finito!
   } else {
      usage();
   }
}

fn parse_n_print(file: impl AsRef<Path>) {
   let lines = tail(lines_from_file(file));
   println!("date,from,to");
   let mut bag = HashSet::new();
   cont(&mut bag, lines);
}

// mutually recursive functions, because what even are for-loops, anyway? :<

fn cont(bag: &mut HashSet<Asset>, lines: Vec<String>) {
   if !lines.is_empty() {
      let (line, rest) = ht(lines);
      print_trades(bag, &line, rest);
   } else {
      println!("\nAssets:\n");
      print_assets(bag);
   }
}
   

fn print_trades(bag: &mut HashSet<Asset>, line_opt: &Option<String>,
                lines: Vec<String>) {
   if let Some(line) = line_opt { 
      let mut new_bag = match read_csv_swap(line) {
         Ok(trade) => { swap_d(bag, trade, true) },
         Err(msg) =>  { println!("ERROR: {}", msg); bag.clone() }
      };
      cont(&mut new_bag, lines);
   }
}
