use std::collections::HashMap;

use chrono::naive::NaiveDate;

use book::{
   file_utils::extract_date_and_body,
   list_utils::{ht,tail}
};

use crypto::{
   parsers::{
      find_date::find_date,
      kujira_nums::parse_kujira_number
   },
   types::usd::{USD,mk_usd}
};

use crate::types::{
   Quotes,Lines,LiquidationsByDate,Market,
   update_market,id_market
};

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

// ----- Prices --------------------------------------------------------

pub fn read_prices(file: &str) -> Quotes {
   let (_date, lines) = extract_date_and_body(file);
   let mut ans = HashMap::new();

   for line in tail(&lines) {
      if let [asset, monay] = line.split(",").collect::<Vec<_>>().as_slice() {
         let quot: USD = monay.parse()
                  .expect(&format!("Could not parse {monay} to USD"));
         ans.insert(asset.to_string(),quot);
      } else { panic!("Unparseable line in prices: {line}") }
   }
   ans
}

// ----- liquidations --------------------------------------------------

pub fn process_liquidations_by_date(prices: &Quotes, lines: &Lines)
   -> LiquidationsByDate {
   let mut ans = HashMap::new();
   process_liqs(prices, lines, &mut ans);
   ans
}

fn process_liqs(prices: &Quotes, lyns: &Lines, ans: &mut LiquidationsByDate) {
   if let Some((n, date, market, amt)) = process_liquidation(prices, lyns) {
      let mut day = ans.entry(date).or_insert(HashMap::new());
      update_market(id_market, &market, &(1, amt), &mut day);
      process_liqs(prices, &skip(n, &lyns), ans);
   }
}

fn process_liquidation(prices: &Quotes, lines: &Lines)
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

fn skip(n: usize, lines: &Lines) -> Lines {
   let (_, t) = lines.split_at(n);
   t.to_vec()
}

fn find_next_date(idx: usize, lines: &Vec<String>) -> Option<(usize, NaiveDate)> {
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
