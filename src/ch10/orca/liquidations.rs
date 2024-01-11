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
   list_utils::ht,
   utils::get_args
};

use crypto::{
   parsers::{
      find_date::find_date,
      kujira_nums::parse_kujira_number
   },
   types::{
      books::{prices,load_books},
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
      let mrk = load_books(&market);
      let prces = prices(&mrk);
      let lines = lines_from_file(&liquids);
      let jours = process_liquidations_by_date(&prces, &lines);
      print_by_days(&jours);
      okay = true;
   }

   // #[allow(unused_must_use)]
   !okay && usage();
}

// ----- Types --------------------------------------------------

type Market = (String, String);

fn market(m: &Market) -> String {
   let (asset, bid) = m;
   format!("{bid}->{asset}")
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
   -> Option<(usize, NaiveDate, Market, USD)> {
   if let Some((n, date)) = find_next_date(0, &lines) {
      let nl: Vec<String> = skip(n, &lines);
      if let Ok(amt) = parse_kujira_number(&nl) {
         if let (Some(asset), t) = ht(&skip(2, &nl)) {
            if let Some(price) = prices.get(&asset) {
               let amount = mk_usd(price.amount * amt);
               if let Some(bid) = skip(2, &t).first() {
                  Some((7, date, (asset, bid.to_string()), amount))
               } else {
                  panic!("Cannot get bid asset for {amt} {asset}")
               }
            } else { panic!("No price for asset {asset}") }
         } else { panic!("Could not parse asset after amt {amt}!") }
      } else { panic!("Parsing num error at {lines:?}") }
   } else { None }
}

fn skip(n: usize, lines: &Vec<String>) -> Vec<String> {
   let (_, t) = lines.split_at(n);
   t.to_vec()
}

fn find_next_date(idx: usize, lines: &Vec<String>)
   -> Option<(usize, NaiveDate)> {
   if lines.is_empty() { None } else {
      if let (Some(h), t) = ht(lines) {
         let nidx = idx + 1;
         match find_date(&h) {
            Ok(date) => Some((nidx, date)),
            _        => find_next_date(nidx, &t)
         }
      } else { panic!("No line but lines not empty? What the???") }
   }
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
