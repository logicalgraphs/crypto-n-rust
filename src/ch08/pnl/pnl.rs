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
   trades::{expenses,read_csv_swap},
   usd::{mk_usd, no_monay}
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

struct TradeState {
   date: String,
   profit: f32,
   loss: f32,
   fees: f32,
   commission: f32,
   ntrades: u8
}

fn init_trade_state() -> TradeState {
   mk_trade_state(String::new(), 0.0, 0.0, 0.0, 0.0, 0)
}

fn mk_trade_state(date: String, profit: f32, loss: f32, fees: f32,
                  commission: f32, ntrades: u8) -> TradeState {
   TradeState { date, profit, loss, fees, commission, ntrades }
}

fn parse_n_print(p: &Portfolio, file: impl AsRef<Path>) {
   let lines = tail(lines_from_file(file));
   println!("date,from,to");
   cont(&p, lines, init_trade_state());
}

// mutually recursive functions, because what even are for-loops, anyway? :<

fn cont(p: &Portfolio, lines: Vec<String>, state: TradeState) {
   if !lines.is_empty() {
      let (line, rest) = ht(lines);
      print_trades(p, &line, rest, state);
   } else {
      let TradeState { date, profit, loss, fees, commission, ntrades } = state;
      print_portfolio(p);
      println!("\nPnL\n\n@TeamKujira FIN order books: {date}");
      println!("\nProfit: {}", mk_usd(profit));
      println!("Loss: {}", mk_usd(loss));
      let subtotal: f32 = profit - loss;
      println!("subtotal: {}", mk_usd(subtotal));
      println!("fees: {}", mk_usd(fees));
      println!("commission: {}", mk_usd(commission));
      let costs: f32 = fees + commission;
      println!("total costs: {}\n", mk_usd(costs));
      let figure: f32 = subtotal - costs;
      let avg = mk_usd(figure / ntrades as f32);
      let total = mk_usd(figure);
      println!("Total profit (or loss): {total} on {ntrades} trades");
      println!("average: {avg} per trade");
      let lg = "https://github.com/logicalgraphs";
      let pnl_sources = "crypto-n-rust/blob/main/src/ch08/pnl/pnl.rs";
      println!("\npnl sources: {lg}/{pnl_sources}\n\nAssets\n");
   }
}

fn print_trades(p: &Portfolio, line_opt: &Option<String>,
                lines: Vec<String>, state: TradeState) {
   if let Some(line) = line_opt { 
      let TradeState { profit, loss, fees, commission, ntrades, .. } = state;
      let (new_portfolio, sub_pnl, (fs, cs), dt) = match read_csv_swap(line) {
         Ok(trde) => {
            let (p1, u) = execute_d(p, &trde, true);
            (p1, u.amount, expenses(&trde), trde.date)
         },
         Err(msg) =>  {
            println!("ERROR: {}", msg);
            (p.clone(), 0.0, (no_monay(), no_monay()), String::new())
         }
      };
      let new_profit = profit + if sub_pnl > 0.0 { sub_pnl } else { 0.0 };
      let new_loss   = loss + if sub_pnl < 0.0 { -1.0 * sub_pnl } else { 0.0 };
      let state1 =
          mk_trade_state(dt, new_profit, new_loss, fees + fs.amount,
                         commission + cs.amount, ntrades + 1);
      cont(&new_portfolio, lines, state1);
   }
}
