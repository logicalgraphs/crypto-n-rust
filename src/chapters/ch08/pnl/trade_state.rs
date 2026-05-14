// Our own little type for computing profit and loss from trades.

// IM SO PROUD! *sniff

use crypto::types::{
   trades::Trade,
   usd::USD
};

// ----- TradeState -------------------------------------------------------

#[derive(Debug, Clone)]
pub struct TradeState {
   date: String,
   profit: f32,
   loss: f32,
   fees: f32,
   commission: f32,
   trades: Vec<Trade>
}

pub fn destructure(s: &TradeState) -> (String, f32, f32, f32, f32, Vec<Trade>) {
   let TradeState { date, profit, loss, fees, commission, trades } = s;
   (date.clone(), *profit, *loss, *fees, *commission, trades.clone())
}

pub fn last_date(s: &TradeState) -> String {
   s.date.clone()
}

pub fn trades(s: &TradeState) -> Vec<Trade> {
   s.trades.clone()
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
