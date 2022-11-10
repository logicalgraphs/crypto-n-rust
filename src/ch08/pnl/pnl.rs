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
   portfolio::{Portfolio,assets_from_file, print_portfolio,execute_d},
   trades::read_csv_swap,
   usd::mk_usd
};

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
   let lines = tail(lines_from_file(file));
   println!("date,from,to");
   cont(&p, lines, 0.0, 0.0);
}

// mutually recursive functions, because what even are for-loops, anyway? :<

fn cont(p: &Portfolio, lines: Vec<String>, profit: f32, loss: f32) {
   if !lines.is_empty() {
      let (line, rest) = ht(lines);
      print_trades(p, &line, rest, profit, loss);
   } else {
      print_portfolio(p);
      println!("Total profit: {}", mk_usd(profit));
      println!("Total loss: {}", mk_usd(loss));
      println!("Net PnL: {}", mk_usd(profit + loss));
   }
}

fn print_trades(p: &Portfolio, line_opt: &Option<String>,
                lines: Vec<String>, profit: f32, loss: f32) {
   if let Some(line) = line_opt { 
      let (new_portfolio, sub_pnl) = match read_csv_swap(line) {
         Ok(trade) => { let (p1, u) = execute_d(p, trade, true); (p1, u.amount) },
         Err(msg) =>  { println!("ERROR: {}", msg); (p.clone(), 0.0) }
      };
      let new_profit = profit + if sub_pnl > 0.0 { sub_pnl } else { 0.0 };
      let new_loss   = loss + if sub_pnl < 0.0 { -1.0 * sub_pnl } else { 0.0 };
      cont(&new_portfolio, lines, new_profit, new_loss);
   }
}
