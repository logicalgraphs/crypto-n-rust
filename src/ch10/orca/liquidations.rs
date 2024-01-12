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

// ----- Imports -------------------------------------------------------

use std::collections::HashMap;
use itertools::Itertools;

use chrono::naive::NaiveDate;

use book::{
   file_utils::{lines_from_file,extract_date_and_body},
   list_utils::{ht,tail},
   utils::get_args
};

use crypto::{
   parsers::{
      find_date::find_date,
      kujira_nums::parse_kujira_number
   },
   types::usd::{USD,no_monay,mk_usd,sum_usd}
};

// ----- Main -------------------------------------------------------

fn usage() -> bool {
   println!("./cillaz <prices CSV> <liquidations LSV>");
   println!("\nSlices and dices liquidations on ORCA (by day and by market)");
   true
}

fn main() {
   let mut okay = false;
   if let [prices, liquids] = get_args().as_slice() {
      let prces = read_prices(&prices);
      let lines = lines_from_file(&liquids);
      let jours = process_liquidations_by_date(&prces, &lines);
      report(&jours);
      okay = true;
   }

   // #[allow(unused_must_use)]
   !okay && usage();
}

fn read_prices(file: &str) -> Quotes {
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

// ----- Types --------------------------------------------------

type Market = (String, String);

fn pair(m: &Market) -> String {
   let (asset, bid) = m;
   format!("{bid},{asset}")
}

fn market(m: &Market) -> String {
   let (asset, bid) = m;
   match str_opt(bid) {
      Some(b) => format!("{b}{}", match str_opt(asset) {
         Some(a) => format!("->{a}"),
         None => "".to_string()
      }),
      None => asset.to_string()
   }
}

fn str_opt(s: &str) -> Option<String> {
   if s == "---" { None} else { Some(s.to_string()) }
}

fn erase_bid(m: &Market) -> Market {
   let (a, _) = m;
   (a.to_string(), "---".to_string())
}

fn erase_asset(m: &Market) -> Market {
   let (_, b) = m;
   ("---".to_string(), b.to_string())
}

fn id_market(m: &Market) -> Market {
   m.clone()
}

type Amount = (usize, USD);
type Liquidations = HashMap<Market, Amount>;
type LiquidationsByDate = HashMap<NaiveDate, Liquidations>;
type Quotes = HashMap<String, USD>;
type Lines = Vec<String>;

// ----- processors --------------------------------------------------

fn process_liquidations_by_date(prices: &Quotes, lines: &Lines)
   -> LiquidationsByDate {
   let mut ans = HashMap::new();
   process_liqs(prices, lines, &mut ans);
   ans
}

fn process_liqs(prices: &Quotes, lyns: &Lines, ans: &mut LiquidationsByDate) {
   if let Some((n, date, market, amt)) = process_liquidation(prices, lyns) {
      let day = ans.entry(date).or_insert(HashMap::new());
      let amount = day.entry(market).or_insert((0, no_monay()));
      let (i, m) = amount;
      *amount = (*i+1, sum_usd(&m, &amt));
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

fn find_next_date(idx: usize, lines: &Lines) -> Option<(usize, NaiveDate)> {
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

fn report(jours: &LiquidationsByDate) {
   report_by("Market", id_market, jours);
   report_by("Bid", erase_asset, jours);
   report_by("Asset", erase_bid, jours);
}

fn report_by(title: &str, f: impl Fn(&Market) -> Market,
             jours: &LiquidationsByDate) {
   println!("\nORCA liquidations by {title}");
   let rows = xform(&f, jours);
   print_by_days(&rows);
   let markets = by_market(&rows);
   header("");
   print_liquidations(None, &markets);
}

fn header(prefix: &str) {
   println!("\n{prefix}bid,asset,n,amount ($),market");
}

fn print_by_days(jours: &LiquidationsByDate) {
   header("date,");
   for date in jours.keys().sorted() {
      let liq = &jours[date];
      print_liquidations(Some(&format!("{date}")), liq);
   }
}

fn print_liquidations(prefix: Option<&str>, liq: &Liquidations) {
   let pre = if let Some(pre1) = prefix { format!("{pre1},")
   } else { "".to_string() };
   let mut bag = Vec::new();
   for (mrket, (n,amt)) in liq {
      bag.push((n,(mrket,amt)));
   }
   bag.sort();
   bag.reverse();

   for (n, (mrket, amt)) in bag {
      println!("{pre}{},{n},{amt},{}", pair(mrket), market(mrket));
   }
}

// ----- Morphers --------------------------------------------------

// without the dates, report the markets for the 7-days trailing

fn by_market(jours: &LiquidationsByDate) -> Liquidations {
   let mut markets: Liquidations = HashMap::new();
   for (_, liqui) in jours {
      for (mrk, amt) in liqui {
         update_market(id_market, mrk, amt, &mut markets);
      }
   }
   markets
}

fn xform(f: impl Fn(&Market) -> Market, jours: &LiquidationsByDate)
   -> LiquidationsByDate {
   let mut ans = HashMap::new();
   for (day, liqs) in jours {
      ans.insert(*day, xform1(&f, &liqs));
   }
   ans
}

fn xform1(f: impl Fn(&Market) -> Market, liqs: &Liquidations) -> Liquidations {
   let mut ans = HashMap::new();
   for (mkt, amt) in liqs {
      update_market(&f, mkt, amt, &mut ans);
   }
   ans
}

fn update_market(f: impl Fn(&Market) -> Market, mkt: &Market,
                 a: &Amount, markets: &mut Liquidations) {
   let key = f(mkt);
   let market = markets.entry(key.clone()).or_insert((0, no_monay()));
   let (n1, amt1) = market;
   let (n, amt) = a;
   *market = (*n + *n1, sum_usd(amt, amt1));
}
