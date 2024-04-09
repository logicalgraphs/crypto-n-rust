use book::list_utils::ht;

use crypto::types::{
   portfolio::{Portfolio,execute_d},
   trades::{Trade,mk_trade,read_tsv_swap}
};

use crate::trade_state::{TradeState,mk_trade_state,trades,destructure};

// ----- Parsing --------------------------------------------------

pub fn parse_trades_d(p: &Portfolio, lines: &Vec<String>,
                      state: &TradeState, debug: bool) -> TradeState {
   if !lines.is_empty() {
      let (line, rest) = ht(&lines);
      parse_trade_cont_d(&parse_trades_d, p, &line, &rest, state, debug)
      // you like how I put call_cc in this code?
      // call_cc: call-with-current-continuation
   } else {
      state.clone()
   }
}

type Continuation =
   dyn Fn(&Portfolio, &Vec<String>, &TradeState, bool) -> TradeState;

fn parse_trade_cont_d(cont: &Continuation, p: &Portfolio,
                      line_opt: &Option<String>, lines: &Vec<String>,
                      state: &TradeState, debug: bool) -> TradeState {
   let mut new_trades = trades(state);
   if let Some(line) = line_opt {
      if debug { println!("\nParsing {line}"); }
      let ( _date, profit, loss, _fees, _commis, _trades ) = destructure(state);
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
      cont(&new_portfolio, lines, &state1, debug)
   } else {
      state.clone()
   }
}

fn update_pnl(state: &TradeState, date: String, profit: f32, loss: f32,
              trades: Vec<Trade>) -> TradeState {
   let (_date, _prof, _loss, fees, commission, _trades) = destructure(state);
   mk_trade_state(date, profit, loss, fees, commission, trades)
}
