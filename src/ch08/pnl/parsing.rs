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
