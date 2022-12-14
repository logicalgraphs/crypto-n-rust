// Our own little type for computing profit and loss from trades.

// IM SO PROUD! *sniff

use crypto::types::{
   portfolio::{Portfolio,execute_d},
   trades::{Swap,read_csv_swap,liquidations_count_and_premium},
   usd::{USD,mk_usd}
};

use book::string_utils::plural;

pub struct TradeState {
   date: String,
   profit: f32,
   loss: f32,
   fees: f32,
   commission: f32,
   trades: Vec<Swap>
}

pub fn init_trade_state(last_line_p: Option<String>) -> TradeState {
   if let Some(last_line) = last_line_p {
      let fees_comms: Vec<&str> = last_line.split(',').collect();
      let fc: Vec<&&str> = fees_comms.iter().skip(3).take(2).collect();
      init_trade_state_cont(&last_line, fc)
   } else {
      panic!("Cannot find last line of trades-file")
   }
}

fn init_trade_state_cont(last_line: &str, fc: Vec<&&str>) -> TradeState {
   if let [fee, comm] = fc.as_slice() {
      let trades: Vec<Swap> = Vec::new();
      let fees: USD = fee.parse().expect(&format!("fee {fee}"));
      let comms: USD = comm.parse().expect(&format!("commission {comm}"));
      let date = String::new();
      mk_trade_state(date,0.0,0.0,fees.amount,comms.amount,trades)
   } else {
      panic!("Cannot split out fees and commission from {last_line}.")
   }
}

pub fn mk_trade_state(date: String, profit: f32, loss: f32, fees: f32,
                      commission: f32, trades: Vec<Swap>) -> TradeState {
   TradeState { date, profit, loss, fees, commission, trades }
}

fn update_pnl(state: &TradeState, date: String, profit: f32, loss: f32,
                      trades: Vec<Swap>) -> TradeState {
   let TradeState { fees, commission, .. } = state.clone();
   mk_trade_state(date, profit, loss, *fees, *commission, trades)
}

pub fn report(state: &TradeState) {
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
   let ntrades: u32 = trades.len().try_into().unwrap();
   let avg = mk_usd(figure / ntrades as f32);
   let total = mk_usd(figure);
   println!("Total profit (or loss): {total} on {}", plural(ntrades, "trade"));
   println!("average: {avg} per trade\n");

   let (nliqs, perc) = liquidations_count_and_premium(trades);
   let n: u32 = nliqs.into();
   if nliqs > 0 {
      println!("{} at a {perc} premium (avg)\n", plural(n, "liquidation"));
   }

   let lg = "https://github.com/logicalgraphs";
   let dir = "crypto-n-rust/blob/main/src/ch08/pnl/";
   let src = "pnl_with_liquidations.rs";
   println!("pnl sources: {lg}/{dir}{src}\n\nAssets in play\n");
}

pub fn parse_trade_cont(cont: &dyn Fn(&Portfolio, Vec<String>, TradeState) -> (),
                        p: &Portfolio, line_opt: &Option<String>,
                        lines: Vec<String>, state: TradeState) {
   let mut new_trades = state.trades.clone();
   if let Some(line) = line_opt { 
      println!("\nParsing {line}");
      let TradeState { profit, loss, .. } = state;
      let (new_portfolio, sub_pnl, dt) = match read_csv_swap(line) {
         Ok(trde) => {
            let (p1, u) = execute_d(p, &trde, true);
            new_trades.push(trde.clone());
            (p1, u.amount, trde.date)
         },
         Err(msg) =>  {
            println!("ERROR: {}", msg);
            (p.clone(), 0.0, String::new())
         }
      };
      let new_profit = profit + if sub_pnl > 0.0 { sub_pnl } else { 0.0 };
      let new_loss   = loss + if sub_pnl < 0.0 { -1.0 * sub_pnl } else { 0.0 };
      let state1 =
          update_pnl(&state, dt, new_profit, new_loss, new_trades);
      cont(&new_portfolio, lines, state1);
   }
}
