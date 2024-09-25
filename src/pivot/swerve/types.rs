use std::{
   collections::{HashMap,HashSet},
   fmt, fmt::Debug,
   ops::Sub
};

use bimap::BiMap;
use chrono::{Days,NaiveDate};

use book::{
   csv_utils::{CsvWriter,list_csv},
   err_utils::ErrStr,
   json_utils::{AsJSON,json_list,to_object},
   list_utils::ht,
   num_utils::{minimax_f32,parse_num},
   string_utils::quot,
   table_utils::{Table,row_filter,col,rows,val},
   types::{stamp,Stamped,Tag,untag}
};

extern crate serde;

use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct Quote { usd: f32 }
pub fn mk_quote(usd: f32) -> Quote { Quote { usd } }

impl CsvWriter for Quote {
   fn as_csv(&self) -> String { format!("{}", self.usd) }
   fn ncols(&self) -> usize { 1 }
}

pub type TokenId = String;

#[derive(Debug,Clone,PartialEq,Eq,PartialOrd,Ord,Hash)]
pub struct Token {
   token_type: String,
   token_name: String
}

pub fn mk_token(name: &str) -> Token {
   fn root(s: &str) -> String {
      s.trim_matches(char::is_lowercase).to_string()
   }
   Token { token_type: root(name), token_name: name.to_string() }
}

impl fmt::Display for Token {
   fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      write!(f, "{}", self.token_name)
   }
}

pub type RawPrices = HashMap<TokenId, Quote>;

pub type Pivots = Vec<String>;
pub type PivotDict = BiMap<TokenId, Token>;
pub type PivotTable = Table<NaiveDate, Token, f32>;

pub type Price = ((TokenId, Token), Quote);

// ----- Diffs -------------------------------------------------------

// The point of diffs is to tell me that the token-prices I requested are the
// token-prices I got in the response.

#[derive(PartialEq,Debug,Clone)]
pub enum Diff { MISSING, ADDED }

impl CsvWriter for Diff {
   fn as_csv(&self) -> String {
      (if self == &Diff::MISSING { "missing" } else { "added"}).to_string()
   }
   fn ncols(&self) -> usize { 1 }
}

pub type Diffs = (Diff, Vec<String>);

// ----- for EMA calculations ---------------------------------------------

type R = Stamped<f32>;

#[derive(Debug,Clone)]
pub struct Ratio { r: R }

fn mk_ratio((dt, ratio): (&NaiveDate, &f32)) -> Ratio {
   let r = stamp(dt, ratio);
   Ratio { r }
}

impl AsJSON for Ratio {
   fn as_json(&self) -> String {
      to_object("date ratio",
                &[quot(&format!("{}", &self.r.date)),
                  format!("{:?}", self.r.pack)])
   }
}

#[derive(Clone,Debug)]
pub struct Name {
   pub base: Token,
   pub target: Token
}

impl fmt::Display for Name {
   fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}<->{}", self.base, self.target)
   }
}

fn mk_name(t1: &Token, t2: &Token) -> Name {
   Name { base: t2.clone(), target: t1.clone() }
}

impl CsvWriter for Name {
   fn as_csv(&self) -> String { format!("{},{}", self.target, self.base) }
   fn ncols(&self) -> usize { 2 }
}

fn namei(n: &Name) -> String { quot(&format!("{}/{}", n.target, n.base)) }

#[derive(Debug,Clone)]
pub struct Ratios {
   name: Name,
   ratios: Vec<Ratio>
}

pub fn mk_ratios(t1: &Token, t2: &Token,
                 dates: &Vec<NaiveDate>, ratios: &Vec<f32>) -> Ratios {
   let dt_ratios: Vec<Ratio> =
      dates.into_iter().zip(ratios.into_iter()).map(mk_ratio).collect();
   let name = mk_name(t1, t2);
   Ratios { name, ratios: dt_ratios }
}

impl AsJSON for Ratios {
   fn as_json(&self) -> String {
      to_object("name ratios", &[namei(&self.name), json_list(&self.ratios)])
   }
}

pub struct EMAs {
   name: Name,
   period: usize,
   emas: Vec<EMA>
}

pub struct EMA {
   ratio: Ratio,
   ema: f32
}

fn mk_ema((r, ema): (&Ratio, &f32)) -> EMA {
   EMA { ratio: r.clone(), ema: ema.clone() }
}

impl AsJSON for EMA {
   fn as_json(&self) -> String {
      to_object("date ratio ema",
                &[quot(&format!("{}", &self.ratio.r.date)),
                  format!("{:?}", self.ratio.r.pack),
                  format!("{:?}", self.ema)])
   }
}

pub fn mk_emas(t1: &Token, t2: &Token, period: usize,
               dates: &Vec<NaiveDate>, ratios: &Vec<f32>) -> EMAs {

   // 1. SMAs for the series
   // ... this is best expressed as a comonad, but oh, well!
   // but that's okay. (for-loops are comonads, anyway).

   let mut acc = 0.0;
   let mut smas = Vec::new();
   for (n, v) in ratios.iter().enumerate() {
      acc += v;
      smas.push(acc / ((n + 1) as f32));
   }

   // 2. emas

   let mut prev = 0.0;
   let mut emas0 = Vec::new();
   for (n, (s, r)) in smas.iter().zip(ratios.iter()).enumerate() {
      let ema = if period < n {
         let denom: f32 = 1.0 + (period as f32);
         let smooth: f32 = 2.0 / denom;
         r * smooth + prev * (1.0 - smooth)
      } else { *s };
      emas0.push(ema);
      prev = ema;
   }

   // now, let's make Ratios, and, from thence, we'll make our EMAs

   let rats = mk_ratios(t1, t2, dates, ratios);
   let emas: Vec<EMA> =
      rats.ratios.iter().zip(emas0.iter()).map(mk_ema).collect();
   EMAs { name: rats.name.clone(), period, emas }
}

impl AsJSON for EMAs {
   fn as_json(&self) -> String {
      to_object("name period emas",
                &[namei(&self.name), format!("{}", self.period),
                  json_list(&self.emas)])
   }
}

pub fn calculate_emas(table: &PivotTable, date: &NaiveDate, for_rows: u64,
                      t1: &Token, t2: &Token) -> ErrStr<EMAs> {
   let days = Days::new(for_rows);
   let start = date.sub(days);

   fn in_range(d: &NaiveDate) -> impl Fn(&NaiveDate) -> bool + '_ {
      |date| { date.ge(d) }
   }
   let domain = row_filter(in_range(&start), &table);
   let a = col(&domain, t1).expect(&format!("NO TOKEN NAMED {t1}"));
   let b = col(&domain, t2).expect(&format!("NO TOKEN NAMED {t2}"));

   let ratios: Vec<f32> =
      a.clone().into_iter()
               .zip(b.clone().into_iter())
               .map(|(a,b)| a / b)
               .collect();

   let dates = rows(&domain);
   let emas = mk_emas(t1, t2, 20, &dates, &ratios);
   Ok(emas)
}

// ----- Recommendations --------------------------------------------------

#[derive(Debug,Clone)]
pub struct Rec {
   name: Name,
   pub call: CALL
}

#[derive(Debug,Clone,PartialEq)]
pub enum CALL { BUY, SELL }

impl fmt::Display for CALL {
   fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      write!(f, "{}", if self == &CALL::BUY { "BUY" } else { "SELL" })
   }
}

impl CsvWriter for Rec {
   fn as_csv(&self) -> String {
      format!("{:?},{}", self.call, self.name.as_csv())
   }
   fn ncols(&self) -> usize { 1 + self.name.ncols() }
}

pub fn mk_rec(emas: &EMAs) -> Stamped<Rec> {
   let ema = emas.emas.last().expect("No last row in EMAs");
   let call = if ema.ratio.r.pack > ema.ema { CALL::SELL } else { CALL:: BUY };
   stamp(&ema.ratio.r.date, &Rec { name: emas.name.clone(), call })
}

pub fn rec(table: &PivotTable, date: &NaiveDate, for_rows: u64,
           t1: &Token, t2: &Token) -> ErrStr<(Stamped<Rec>, Option<f32>)> {
   let emas = calculate_emas(table, date, for_rows, t1, t2)?;
   let deltas = mk_deltas(&emas);
   Ok((mk_rec(&emas),confidence(&deltas)))
}

pub fn rec_as_string(r: &Stamped<Rec>) -> String {
   format!("On {}, {} {} {} {}", r.date, r.pack.call, 
           r.pack.name.target,
           if r.pack.call == CALL::BUY { "with" } else { "for" },
           r.pack.name.base)
}

// ----- Deltas -------------------------------------------------------

// Now, eventually, Deltas will tell us exactly how much A to swap for B
// but, for now, they give us a confidence in the swap as a percentage

#[derive(Clone,Debug)]
pub struct Delta { d: R }

impl AsJSON for Delta {
   fn as_json(&self) -> String {
      to_object("date delta",
                &[quot(&format!("{}", &self.d.date)),
                  format!("{:?}", self.d.pack)])
   }
}

impl CsvWriter for Delta {
   fn ncols(&self) -> usize { 2 }
   fn as_csv(&self) -> String { format!("{},{:?}", self.d.date, self.d.pack) }
}

fn mk_delta(ema: &EMA) -> Delta {
   let delta = ema.ema - ema.ratio.r.pack;
   Delta { d: stamp(&ema.ratio.r.date, &delta) }
}

#[derive(Clone,Debug)]
pub struct Deltas { deltas: Vec<Delta> }

pub fn mk_deltas(emas: &EMAs) -> Deltas {
  Deltas { deltas: emas.emas.iter().map(mk_delta).collect() }
}

impl AsJSON for Deltas {
   fn as_json(&self) -> String {
      to_object("deltas", &[json_list(&self.deltas)])
   } 
}  

impl CsvWriter for Deltas {
   fn ncols(&self) -> usize { 2 }
   fn as_csv(&self) -> String {
      format!("date,delta\n{}", list_csv(&self.deltas))
   }
}

pub fn confidence(ds: &Deltas) -> Option<f32> {
   ds.deltas.last().and_then(|stamped_delta| {
      let deltas: Vec<f32> = ds.deltas.iter().map(|st_d| st_d.d.pack).collect();
      let (mb_min, mb_max) = minimax_f32(&deltas);
      mb_min.and_then(|min| {
         mb_max.and_then(|max| {
            let d = stamped_delta.d.pack;
            let conf = d / if d > 0.0 { max } else { min };
            Some(conf)
         })
      })
   })
}

pub fn print_confidence(dt: &NaiveDate, mb_conf: &Option<f32>) {
   println!("{}", if let Some(conf) = mb_conf {
      format!("Confidence for {dt} trade: {:.2?}%", conf * 100.0)
   } else {
      "no confidence".to_string()
   });
}

// ... and the application of deltas to assets

type Blockchain = String;
type Amount = f32;
type Tokens = HashMap<Token, Amount>;
type Prime = Option<Token>;

fn parse_tokens(row: &Vec<String>) -> ErrStr<(Tokens, Prime)> {
   let mut ans = HashMap::new();
   let mut prime = None;
   for window in row.chunks(2) {
      if let Some(tok) = window.get(0) {
         if tok == "" { continue; }
         let tok1 = if tok.starts_with("*") {
            let ans = tok.strip_prefix("*").unwrap();
            prime = Some(mk_token(&ans));
            ans
         } else {
            tok
         };
         let token = mk_token(&tok1);
         let amt = window.get(1).ok_or(format!("No amount listed for {tok}"))?;
         let amount = parse_num(&amt)?;
         ans.insert(token, amount);
      }
   }
   Ok((ans, prime))
}

pub struct Assets {
   blockchain: Blockchain,
   tokens: Tokens,
   prime: Prime
}

pub fn asset_parser(v: Vec<String>) -> ErrStr<Assets> {
   let (h, t) = ht(&v);
   let blockchain = h.ok_or(format!("No blockchain in {v:?}"))?;
   let (tokens, prime) = parse_tokens(&t)?;
   Ok(Assets { blockchain, tokens, prime })
}

pub type Pools = HashMap<Blockchain, Vec<(Prime, Tokens)>>;

pub fn build_pools(blocks: &Vec<Assets>) -> Pools {
   let mut ans = HashMap::new();
   for block in blocks {
      fn toks(b: &Assets) -> (Prime, Tokens) {
         (b.prime.clone(), b.tokens.clone())
      }
      ans.entry(block.blockchain.clone())
         .and_modify(|assets: &mut Vec<_>| assets.push(toks(&block)))
         .or_insert(vec!(toks(&block)));
   }     
   ans
}

// ----- Trade-Routes --------------------------------------------------

pub type TradeRoute = Name;

pub fn build_trade_routes(mb_prime: &Prime, a: &Tokens) -> Vec<TradeRoute> {
   let mut toks: HashSet<Token> = a.keys().cloned().collect();
   fn vectorize(h: HashSet<Token>) -> Vec<Token> { h.into_iter().collect() }
   if let Some(prime) = mb_prime {
      toks.remove(&prime);
      build_trade_routes_with(mb_prime.clone(), vectorize(toks))
   } else {
      fix_build_trade_routes(vectorize(toks))
   }
}

fn fix_build_trade_routes(v: Vec<Token>) -> Vec<TradeRoute> {
   let (a, b) = ht(&v);
   let mut ans = build_trade_routes_with(a, b.clone());
   if ans.is_empty() {
      ans
   } else { 
      ans.append(&mut fix_build_trade_routes(b));
      ans
   }
}

fn build_trade_routes_with(t: Option<Token>, v: Vec<Token>)
      -> Vec<TradeRoute> {
   if let Some(tok) = t {
      fn mk_trade_route(base: Token) -> impl Fn(Token) -> TradeRoute {
         move |target| mk_name(&base, &target)
      }
      v.into_iter().map(mk_trade_route(tok)).collect()
   } else {
      Vec::new()
   }
}

// ----- Chart-data, or, fetching historical data for tokens -----------------

pub type StampedData<A> = HashMap<NaiveDate, A>;
pub type Chart<A> = HashMap<String, StampedData<A>>;

pub fn print_chart<A: Debug + Clone>(c: &Tag<Chart<A>>) {
   let (tag, value) = untag(&c);
   println!("{} Chart:\n", tag);
   for section in value {
      print_section(&section);
   }
}

fn print_section<A: Debug + Clone>((section, row): &(String, StampedData<A>)) {
   println!("Section: {section}");

   fn print_datum<A: Debug>(data: &A) {
      println!("\t{:?}", data);
   }
   let mut prices: Vec<(NaiveDate, A)> = Vec::new();
   // ugh: row.into_iter().cloned().collect();
   for (k,v) in row { prices.push((k.clone(), v.clone())); }
   prices.sort_by_key(|k| k.0);
   prices.iter().take(3).for_each(print_datum);
   println!("\t...");
}

// ----- Trade-Call -------------------------------------------------------

// Given a set of Tokens(-with-amounts) and the pivot-table, we build the
// EMA-20s, the deltas, and a recommendation.

// From those derived values we can compute amount (and type) to trade and
// the amount (of type) we'd receive.

#[derive(Debug, Clone)]
pub struct PricedAsset {
   token: Token,
   amount: f32,
   price: f32
}

impl fmt::Display for PricedAsset {
   fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      write!(f, "{} {} @ ${}ea (${})",
             self.amount, self.token, self.price, self.amount * self.price)
   }
}

#[derive(Debug, Clone)]
pub struct TradeCall {
   from: PricedAsset,
   to:   PricedAsset
}

impl fmt::Display for TradeCall {
   fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      write!(f, "SWAP {} for {}", self.from, self.to)
   }
}

pub fn mk_trade_call(table: &PivotTable, date: &NaiveDate, for_rows: u64,
                     amounts: &Tokens, route: &TradeRoute, min_amt: f32)
      -> ErrStr<Option<TradeCall>> {
   let (rec, conf) = rec(table, date, for_rows, &route.base, &route.target)?;
   if let Some(delta) = conf {
      let (from_t, to_t) = if rec.pack.call == CALL::SELL {
         (route.base.clone(), route.target.clone())
      } else {
         (route.target.clone(), route.base.clone())
      };
      fn adjust_amt(toks: &Tokens, t: &Token, c: f32) -> ErrStr<f32> {
         let amt = toks.get(t).expect(&format!("No amount for {t}"));
         Ok(amt * 0.1 * c)
      }
      let from_amt = adjust_amt(amounts, &from_t, delta)?;
      fn tok_price(tt: &PivotTable, t: &Token, dt: &NaiveDate) -> ErrStr<f32> {
         val(tt, dt, t).ok_or(format!("No price for token {t} on {dt}"))
      }
      let from_prc = tok_price(table, &from_t, date)?;
      if from_prc * from_amt < min_amt {
         Ok(None)
      } else {
         let to_prc = tok_price(table, &to_t, date)?;
         let to_amt = from_prc * from_amt / to_prc;
         let from =
            PricedAsset { token: from_t, amount: from_amt, price: from_prc };
         let to =
            PricedAsset { token: to_t, amount: to_amt, price: to_prc };
         Ok(Some(TradeCall { from, to }))
      }
   } else {
      Ok(None)
   }
}
