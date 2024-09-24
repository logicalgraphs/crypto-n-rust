use std::{
   collections::HashMap,
   fmt, fmt::Debug
};

use bimap::BiMap;
use chrono::NaiveDate;

use book::{
   csv_utils::{CsvWriter,list_csv},
   err_utils::ErrStr,
   json_utils::{AsJSON,json_list,to_object},
   list_utils::ht,
   num_utils::{minimax_f32,parse_num},
   string_utils::quot,
   table_utils::Table,
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
struct Name {
   base: Token,
   target: Token
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

// ----- Recommendations --------------------------------------------------

#[derive(Debug,Clone)]
pub struct Rec {
   name: Name,
   call: CALL
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

pub fn rec(r: &Stamped<Rec>) -> String {
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
   }).or(None)
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
      fn toks(b: &Assets) -> (Prime, Tokens) { (b.prime.clone(), b.tokens.clone()) }
      ans.entry(block.blockchain.clone())
         .and_modify(|assets: &mut Vec<_>| assets.push(toks(&block)))
         .or_insert(vec!(toks(&block)));
   }     
   ans
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
