// computes profit and loss (pnl) from trades

use std::{
   path::Path
};

use book::{
   utils::get_args,
   file_utils::lines_from_file,
   list_utils::{tail,ht}
};

use crypto::types::{
   portfolio::{Portfolio,assets_from_file,print_portfolio}
};

mod trade_state;
use crate::trade_state::{TradeState,init_trade_state,report,parse_trade_cont};

fn usage() {
   println!("\n./pnl <assets CSV file> <trades CSV file>");
   println!("\n\tprints the profits and losses from trades.");
}

fn main() {
   if let [assets, trades] = get_args().as_slice() {
      let starboard = assets_from_file(assets);
      parse_n_print(&starboard, trades);
      println!("Finito!");  // a little Italian flourish at the finito!
   } else {
      usage();
   }
}

fn parse_n_print(p: &Portfolio, file: impl AsRef<Path>) {
   let mut lines = tail(lines_from_file(file));
   let trade_state = init_trade_state(lines.pop());
   cont(&p, lines, trade_state);
}

// mutually recursive functions, because what even are for-loops, anyway? :<

fn cont(p: &Portfolio, lines: Vec<String>, state: TradeState) {
   if !lines.is_empty() {
      let (line, rest) = ht(lines);
      parse_trade_cont(&cont, p, &line, rest, state);
   } else {
      print_portfolio(p);
      report(&state);
   }
}
