// computes profit and loss (pnl) from trades

use book::{
   utils::get_args,
   file_utils::lines_from_file,
   list_utils::{tail,ht,first_last}
};

use crypto::types::portfolio::{Portfolio,assets_from_file,print_portfolio};

use pnl::{
   parsing::parse_trades_d,
   reports::{report,enumerate_trades},
   trade_state::init_trade_state
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
         } else { (Some(first_arg), rest.first()) } {
         help = false;
         let starboard = assets_from_file(&assets);
         parse_then_print(&starboard, &trades, debug);
         println!("Finito!");  // a little Italian flourish at the finito!
      }
   }

   if help {
      usage();
   }
}

fn parse_then_print(p: &Portfolio, file: &str, debug: bool) {
   let mut lines = tail(&lines_from_file(file));
   let state0 = init_trade_state(lines.pop());
   let state = parse_trades_d(&p, &lines, &state0, debug);
   if debug { enumerate_trades(&state); }
   print_portfolio(p);
   report(&state);
}
