use std::collections::HashMap;

use book::{
   csv_utils::list_csv,
   list_utils::first_last,
   string_utils::plural
};

use crypto::types::{
   trades::{Trade,trade,liquidations_count_and_premium},
   usd::{USD,mk_usd,no_monay,sum_usd}
};

use crate::trade_state::{TradeState,destructure,trades};

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

   coalesce_trades(&trades);

   let lg = "https://github.com/logicalgraphs";
   let dir = "crypto-n-rust/blob/main/src/ch08/pnl/";
   let src = "pnl_with_liquidations.rs";
   println!("pnl sources: {lg}/{dir}{src}\n\nAssets in play\n");
}

pub fn enumerate_trades(ts: &TradeState) {
   println!("row,date,sell,amt,quote,buy,amt,quote,premium,pnl");
   println!("{}", list_csv(&trades(&ts)));
}

fn coalesce_trades(t: &Vec<Trade>) {
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
   if let Some(prof) = hi {
      println!("Most profitable asset traded: {} for {}", prof.0, prof.1);
   }
   if let Some(loss) = lo {
      println!("Most lossy asset traded: {} for {}", loss.0, loss.1);
   }
   println!("");
}
