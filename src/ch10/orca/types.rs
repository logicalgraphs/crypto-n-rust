use std::collections::HashMap;

use chrono::naive::NaiveDate;

use book::csv_utils::{CsvWriter,Stamped,stamp,Indexed,mk_idx,mk_idx_offset};
use crypto::types::usd::{USD,no_monay,sum_usd};

// ----- Types -------------------------------------------------------

pub type Market = (String, String);

pub fn pair(m: &Market) -> String {
   let (asset, bid) = m;
   format!("{bid},{asset}")
}

pub fn market(m: &Market) -> String {
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

pub fn erase_bid(m: &Market) -> Market {
   let (a, _) = m;
   (a.to_string(), "---".to_string())
}

pub fn erase_asset(m: &Market) -> Market {
   let (_, b) = m;
   ("---".to_string(), b.to_string())
}

pub fn id_market(m: &Market) -> Market {
   m.clone()
}

pub type Amount = (usize, USD);
pub type Liquidations = HashMap<Market, Amount>;

#[derive(Debug, Clone)]
pub struct Liquidation {
   market: Market,
   amount: Amount
}

pub fn mk_liquidation(pear: (&Market, &Amount)) -> Liquidation {
   let (m, a) = pear;
   Liquidation { market: m.clone(), amount: a.clone() }
}

impl CsvWriter for Liquidation {
   fn as_csv(&self) -> String {
      let (cnt,amt) = self.amount;
      format!("{},{},{},{}",
              pair(&self.market), cnt, amt, market(&self.market))
   }
   fn ncols(&self) -> usize { 2 + 2 + 1 }
}

pub type LiquidationsByDate = HashMap<NaiveDate, Liquidations>;
pub type Quotes = HashMap<String, USD>;
pub type Lines = Vec<String>;

#[derive(Debug, Clone)]
pub struct Top5 {
   share: USD,
   market: Market
}

fn mk_top5(share: &USD, market: &Market) -> Top5 {
   Top5 { share: share.clone(), market: market.clone() }
}

impl CsvWriter for Top5 {
   fn as_csv(&self) -> String {
      format!("{},for,{}", market(&self.market), self.share)
   }
   fn ncols(&self) -> usize { 1 + 2 }
}

pub type Top5s = Vec<Indexed<Top5>>;

// ----- Transformers -------------------------------------------------------

pub fn by_market(f: impl Fn(&Market) -> Market, jours: &LiquidationsByDate)
   -> Liquidations {
   let mut markets: Liquidations = HashMap::new();
   for (_, liqui) in jours {
      for (mrk, amt) in liqui {
         update_market(&f, mrk, amt, &mut markets);
      }
   }
   markets
}

pub fn big_generator(ls: &LiquidationsByDate) -> Vec<Stamped<Liquidation>> {
   let mut ans = Vec::new();
   for (date, liqs) in ls {
      for (mkt, amt) in liqs {
         ans.push(stamp(date, &mk_liquidation((mkt, amt))));
      }
   }
   ans.sort_by(|a,b| b.date.cmp(&a.date));
   ans
}

pub fn liquidations(ls: &LiquidationsByDate) -> Vec<Indexed<Liquidation>> {
   let liqs = by_market(id_market, ls);
   let mut ans = Vec::new();
   for (mkt, (n, amt)) in liqs {
      ans.push(mk_idx(n, &mk_liquidation((&mkt, &(n, amt)))));
   }
   ans.sort_by(|a, b| b.idx.cmp(&a.idx));
   ans
}

pub fn top5s(liqs: &Vec<Indexed<Liquidation>>) -> Top5s {
   let mut tops = Vec::new();
   for idxd in liqs {
      let Liquidation { market, amount } = &idxd.pack;
      let (_, amt) = amount;
      tops.push(mk_top5(&amt, &market));
   }
   tops.sort_by(|a, b| b.share.cmp(&a.share));
   tops.iter().take(5).enumerate().map(mk_idx_offset).collect()
}

// ----- Helpers -------------------------------------------------------

pub fn xform(f: impl Fn(&Market) -> Market, jours: &LiquidationsByDate)
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

pub fn update_market(f: impl Fn(&Market) -> Market, mkt: &Market,
                 a: &Amount, markets: &mut Liquidations) {
   let key = f(mkt);
   let market = markets.entry(key).or_insert((0, no_monay()));
   let (n1, amt1) = market;
   let (n, amt) = a;
   *market = (*n + *n1, sum_usd(amt, amt1));
}

