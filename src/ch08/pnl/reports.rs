use std::collections::HashMap;

use book::{
   csv_utils::list_csv,
   list_utils::first_last,
   num_utils::mk_estimate,
   string_utils::plural
};

use crypto::types::{
   trades::{Trade,trade,trade_date,liquidations_count_and_premium},
   usd::{USD,mk_usd,no_monay,sum_usd}
};

use crate::trade_state::{TradeState,destructure,trades,last_date};

// ----- Reporting --------------------------------------------------

pub fn report(state: &TradeState) {
   let (date, profit, loss, fees, commission, trades) = destructure(&state);
   println!("\n{date} @TeamKujira FIN order books: PnL");

   println!("\nProfit: {}", mk_usd(profit));
   println!("Loss: {}", mk_usd(loss));
   let subtotal: f32 = profit - loss;
   println!("subtotal: {}", mk_usd(subtotal));
   println!("fees: {}", mk_usd(fees));
   println!("commission: {}", mk_usd(commission));
   let costs: f32 = fees + commission;
   println!("total costs: {}\n", mk_usd(costs));
   let figure: f32 = subtotal - costs;
   let ntrades: u32 = trades.len().try_into().unwrap();
   let avg = mk_usd(figure / ntrades as f32);
   let total = mk_usd(figure);
   println!("Total profit (or loss): {total} on {}", plural(ntrades, "trade"));
   println!("average: {avg} per trade\n");

   let (nliqs, perc) = liquidations_count_and_premium(&trades);
   let n: u32 = nliqs.into();
   if nliqs > 0 {
      println!("{} at a {perc} premium (avg)\n", plural(n, "liquidation"));
   }

   coalesce_trades(&date, &trades);

   let lg = "https://github.com/logicalgraphs";
   let dir = "crypto-n-rust/blob/main/src/ch08/pnl/";
   let src = "pnl_with_liquidations.rs";
   println!("pnl sources: {lg}/{dir}{src}\n");
}

pub fn enumerate_trades(ts: &TradeState) {
   println!("row,date,sell,amt,quote,buy,amt,quote,premium,pnl");
   println!("{}", list_csv(&trades(&ts)));
   trade_analysis(ts);
}

fn trade_analysis(ts: &TradeState) {
   println!("\n{} @TeamKujira PnL Analysis\n", last_date(ts));
   let mut all_days = HashMap::new();
   let mut pnl = no_monay();
   let trds = trades(&ts);
   for trade in &trds {
      let day = all_days.entry(trade_date(trade)).or_insert((0, no_monay()));
      *day = (day.0 + 1, sum_usd(&day.1, &trade.pnl));
      pnl = sum_usd(&pnl, &trade.pnl);
   }

   let ntrades = trds.len() as f32;
   let ndays = all_days.len() as f32;
   println!("Number of days traded: {ndays}");
   println!("Average number of trades/day: {}", mk_estimate(ntrades / ndays));
   println!("Average PnL/day: {}\n", mk_usd(pnl.amount / ndays));

   let mut days: Vec<(String, (i32, USD))> = all_days.into_iter().collect();
   days.sort_by(|a, b| a.1.0.cmp(&b.1.0));
   trading_day("Most active", days.last());

   days.sort_by(|a, b| a.1.1.cmp(&b.1.1));
   trading_day("Most profitable", days.last());
   trading_day("Least profitable", days.first());
}

type DailyPnL = (String, (i32, USD));

fn trading_day(kind: &str, day: Option<&DailyPnL>) {
   if let Some((date, (n, pnl))) = day {
      println!("{kind} trading day: {date} with {n} trades, pnl: {pnl}");
   }
}

fn coalesce_trades(date: &String, t: &Vec<Trade>) {
   let mut all_trades = HashMap::new();
   for tr in t {
     let acc = all_trades.entry(trade(tr)).or_insert(no_monay());
     *acc = sum_usd(acc, &tr.pnl);
   }
   fn tolerate(p: &(String, USD)) -> bool {
      let val = p.1.amount;
      val > 99.9 || val < -10.0
   }
   let mut trades: Vec<(String, USD)> =
      all_trades.into_iter().filter(tolerate).collect();
   trades.sort_by(|a, b| b.1.cmp(&a.1));
   println!("trade,pnl");
   for (k,v) in &trades {
      println!("{k},{v}");
   }
   println!("");

   let (hi,lo) = first_last(&trades);
   println!("{date} Kujira FIN pnl vs. trades\n");

   if let Some(prof) = hi {
      println!("Most profitable asset traded: {} for {}", prof.0, prof.1);
   }
   if let Some(loss) = lo {
      println!("Most lossy asset traded: {} for {}", loss.0, loss.1);
   }
   println!("");
}
