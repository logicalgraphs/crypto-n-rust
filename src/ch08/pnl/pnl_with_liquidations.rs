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
   trades::{expenses,read_csv_swap,Swap,liquidations_count_and_premium},
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
   trades: Vec<Swap>
}

fn init_trade_state() -> TradeState {
   let trades: Vec<Swap> = Vec::new();
   mk_trade_state(String::new(), 0.0, 0.0, 0.0, 0.0, trades)
}

fn mk_trade_state(date: String, profit: f32, loss: f32, fees: f32,
                  commission: f32, trades: Vec<Swap>) -> TradeState {
   TradeState { date, profit, loss, fees, commission, trades }
}

fn parse_n_print(p: &Portfolio, file: impl AsRef<Path>) {
   let lines = tail(lines_from_file(file));
   cont(&p, lines, init_trade_state());
}

// mutually recursive functions, because what even are for-loops, anyway? :<

fn cont(p: &Portfolio, lines: Vec<String>, state: TradeState) {
   if !lines.is_empty() {
      let (line, rest) = ht(lines);
      print_trade(p, &line, rest, state);
   } else {
      print_portfolio(p);
      report(&state);
   }
}

fn report(state: &TradeState) {
   let TradeState { date, profit, loss, fees, commission, trades } = state;
   println!("\nPnL\n\n@TeamKujira FIN order books: {date}");

   println!("\nProfit: {}", mk_usd(*profit));
   println!("Loss: {}", mk_usd(*loss));
   let subtotal: f32 = profit - loss;
   println!("subtotal: {}", mk_usd(subtotal));
   println!("fees: {}", mk_usd(*fees));
   println!("commission: {}", mk_usd(*commission));
   let costs: f32 = fees + commission;
   println!("total costs: {}\n", mk_usd(costs));
   let figure: f32 = subtotal - costs;
   let ntrades = trades.len();
   let avg = mk_usd(figure / ntrades as f32);
   let total = mk_usd(figure);
   println!("Total profit (or loss): {total} on {ntrades} trades");
   println!("average: {avg} per trade\n");

   let (nliqs, perc) = liquidations_count_and_premium(trades);
   if nliqs > 0 {
      let s = if nliqs == 1 { "" } else { "s" };
      println!("{nliqs} liquidation{s} at a {perc} premium (avg)\n");
   }

   let lg = "https://github.com/logicalgraphs";
   let dir = "crypto-n-rust/blob/main/src/ch08/pnl/";
   let src = "pnl_with_liquidatios.rs";
   println!("pnl sources: {lg}/{dir}{src}\n\nAssets\n");
}

fn print_trade(p: &Portfolio, line_opt: &Option<String>,
               lines: Vec<String>, state: TradeState) {
   let mut new_trades = state.trades.clone();
   if let Some(line) = line_opt { 
      println!("Parsing {line}");
      let TradeState { profit, loss, fees, commission, .. } = state;
      let (new_portfolio, sub_pnl, (fs, cs), dt) = match read_csv_swap(line) {
         Ok(trde) => {
            let (p1, u) = execute_d(p, &trde, true);
            new_trades.push(trde.clone());
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
                         commission + cs.amount, new_trades);
      cont(&new_portfolio, lines, state1);
   }
}
