// computes profit and loss (pnl) from trades

use book::{
   utils::get_args,
   file_utils::lines_from_file,
   list_utils::{tail,ht}
};

use crypto::types::{
   portfolio::{Portfolio,assets_from_file,print_portfolio}
};

mod trade_state;
use crate::trade_state::{
   TradeState,init_trade_state,report,parse_trade_cont_d,print_trades
};

fn usage() {
   println!("\n./pnl [-v] <assets CSV file> <trades TSV file>");
   println!("\n\tprints the profits and losses from trades.");
   println!("\n\tthe -v option spews (a lot!) of debug information.\n");
}

fn main() {
   let mut help = true;
   if let (Some(first_arg), rest) = ht(&get_args()) {
      let debug = first_arg == "-v";
      if let (Some(assets), Some(trades)) =
         if debug { first_last(&rest)
         } else { (Some(&first_arg), rest.first()) } {
         help = false;
         let starboard = assets_from_file(assets);
         parse_n_print(&starboard, trades, debug);
         println!("Finito!");  // a little Italian flourish at the finito!
      }
   }

   if help {
      usage();
   }
}

fn first_last(v: &Vec<String>) -> (Option<&String>, Option<&String>) {
   (v.first(), v.last())
}

fn parse_n_print(p: &Portfolio, file: &str, debug: bool) {
   let mut lines = tail(&lines_from_file(file));
   let trade_state = init_trade_state(lines.pop());
   cont_d(&p, &lines, &trade_state, debug);
}

// mutually recursive functions, because what even are for-loops, anyway? :<

fn cont_d(p: &Portfolio, lines: &Vec<String>, state: &TradeState, debug: bool) {
   if !lines.is_empty() {
      let (line, rest) = ht(&lines);
      parse_trade_cont_d(&cont_d, p, &line, &rest, state, debug);
      // you like how I put call_cc in this code?
      // call_cc: call-with-current-continuation
   } else {
      if debug { print_trades(state); }
      print_portfolio(p);
      report(&state);
   }
}
