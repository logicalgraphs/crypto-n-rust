// Our own little type for computing profit and loss from trades.

// IM SO PROUD! *sniff

use std::collections::HashMap;

use crypto::types::{
   portfolio::{Portfolio,execute_d},
   trades::{Trade,mk_trade,read_tsv_swap,liquidations_count_and_premium,trade},
   usd::{USD,mk_usd,no_monay,sum_usd}
};

use book::{
   csv_utils::list_csv,
   string_utils::plural
};

pub struct TradeState {
   date: String,
   profit: f32,
   loss: f32,
   fees: f32,
   commission: f32,
   trades: Vec<Trade>
}

pub fn init_trade_state(last_line_p: Option<String>) -> TradeState {
   if let Some(last_line) = last_line_p {
      let fees_comms: Vec<&str> = last_line.split('\t').collect();
      let fc: Vec<&&str> = fees_comms.iter()
                                     .filter(|x| !x.is_empty())
                                     .skip(1).take(2).collect();
      init_trade_state_cont(&last_line, fc)
   } else {
      panic!("Cannot find last line of trades-file")
   }
}

fn init_trade_state_cont(last_line: &str, fc: Vec<&&str>) -> TradeState {
   if let [fee, comm] = fc.as_slice() {
      let trades: Vec<Trade> = Vec::new();
      let fees: USD = fee.parse().expect(&format!("fee {fee}"));
      let comms: USD = comm.parse().expect(&format!("commission {comm}"));
      let date = String::new();
      mk_trade_state(date,0.0,0.0,fees.amount,comms.amount,trades)
   } else {
      panic!("Cannot extract fees and commission from {last_line}; fc {fc:?}")
   }
}

pub fn mk_trade_state(date: String, profit: f32, loss: f32, fees: f32,
                      commission: f32, trades: Vec<Trade>) -> TradeState {
   TradeState { date, profit, loss, fees, commission, trades }
}

fn update_pnl(state: &TradeState, date: String, profit: f32, loss: f32,
              trades: Vec<Trade>) -> TradeState {
   let TradeState { fees, commission, .. } = state;
   mk_trade_state(date, profit, loss, *fees, *commission, trades)
}

// ----- Reporting --------------------------------------------------

pub fn report(state: &TradeState) {
   let TradeState { date, profit, loss, fees, commission, trades } = state;
   println!("\n{date} @TeamKujira FIN order books: PnL");

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

   coalesce_trades(trades);

   let lg = "https://github.com/logicalgraphs";
   let dir = "crypto-n-rust/blob/main/src/ch08/pnl/";
   let src = "pnl_with_liquidations.rs";
   println!("pnl sources: {lg}/{dir}{src}\n\nAssets in play\n");
}

pub fn enumerate_trades(ts: &TradeState) {
   println!("row,date,sell,amt,quote,buy,amt,quote,premium,pnl");
   println!("{}", list_csv(&ts.trades));
}

fn coalesce_trades(t: &Vec<Trade>) {
   let mut trades = HashMap::new();
   for tr in t {
     let acc = trades.entry(trade(tr)).or_insert(no_monay());
     *acc = sum_usd(acc, &tr.pnl);
   }
   println!("trade,pnl");
   for (k,v) in &trades {
      println!("{k},{v}");
   }
   println!("");
}

// ----- Parsing --------------------------------------------------

type Continuation = dyn Fn(&Portfolio, &Vec<String>, &TradeState, bool) -> ();

pub fn parse_trade_cont_d(cont: &Continuation, p: &Portfolio,
                         line_opt: &Option<String>, lines: &Vec<String>,
                         state: &TradeState, debug: bool) {
   let mut new_trades = state.trades.clone();
   if let Some(line) = line_opt { 
      if debug { println!("\nParsing {line}"); }
      let TradeState { profit, loss, .. } = state;
      let (new_portfolio, sub_pnl, dt) = match read_tsv_swap(line) {
         Ok(trde) => {
            let (p1, u) = execute_d(p, &trde, debug);
            new_trades.push(mk_trade(trde.clone(), u.clone()));
            (p1, u.amount, trde.date)
         },
         Err(msg) =>  {
            println!("ERROR: {msg}");
            (p.clone(), 0.0, String::new())
         }
      };
      let new_profit = profit + if sub_pnl > 0.0 { sub_pnl } else { 0.0 };
      let new_loss   = loss + if sub_pnl < 0.0 { -1.0 * sub_pnl } else { 0.0 };
      let state1 =
          update_pnl(&state, dt, new_profit, new_loss, new_trades);
      cont(&new_portfolio, lines, &state1, debug);
   }
}
