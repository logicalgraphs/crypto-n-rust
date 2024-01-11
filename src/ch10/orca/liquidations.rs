/*
ORCA liquidations follow the following format per liquidation

1. date field ([sym] <date> <time>)
2+3. collateral liquidated
4. collateral sym
5+6. amount paid
7. paid with sym
8+9. avg price
10. avg price denom
11. nbsp
12. premium

The very next line is the next liquidation.
*/

use std::collections::HashMap;

use chrono::naive::NaiveDate;

use book::{
   file_utils::lines_from_file,
   list_utils::tail,
   utils::get_args
};

use crypto::{
   algos::orders::read_marketplace,
   types::{
      books::prices,
      usd::{USD,no_monay,mk_usd}
   }
};

fn usage() -> bool {
   println!("./cillaz <market JSON> <liquidations LSV>");
   println!("\nSlices and dices liquidations on ORCA (by day and by market)");
   true
}

fn main() {
   let mut okay = false;
   if let [market, liquids] = get_args().as_slice() {
      let mrk = read_marketplace(&market);
      let prces = prices(&mrk);
      let lines = tail(&lines_from_file(&liquids));
      let jours = process_liquidations_by_date(&prces, &lines);
      print_by_days(&jours);
      okay = true;
   }

   #[allow(unused_must_use)]
   !okay && usage();
}

// ----- Types --------------------------------------------------

type Market = (String, String);

fn market(m: &Market) -> String {
   let (asset, collateral) = m;
   format!("{collateral}->{asset}")
}

type Amount = (usize, USD);
type Liquidations = HashMap<Market, Amount>;

// ----- processors --------------------------------------------------

fn process_liquidations_by_date(prices: &HashMap<String, USD>,
                                lines: &Vec<String>)
   -> HashMap<NaiveDate, Liquidations> {
   HashMap::new()
}

fn process_liquidation(prices: &HashMap<String, USD>, lines: &Vec<String>)
   -> Option<(usize, Market, USD)> {
   if let Some(idx, mark, amt) = find_next_date(&lines):
}

// ----- Printers --------------------------------------------------

fn print_by_days(jours: &HashMap<NaiveDate, Liquidations>) {
   for (date,liq) in jours {
      print_liquidations(Some(&format!("{date}")), liq);
   }
}

fn print_liquidations(prefix: Option<&str>, liq: &Liquidations) {
   let pre = if let Some(pre1) = prefix { format!("{pre1},")
   } else { "".to_string() };
   for (mrket, (n,amt)) in liq {
      println!("{pre}{},{n},{amt}", market(mrket));
   }
}
