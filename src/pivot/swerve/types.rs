use std::{
   collections::HashMap,
   fmt
};

use chrono::NaiveDate;

use book::{
   csv_utils::{CsvWriter,list_csv},
   json_utils::{AsJSON,json_list,to_object},
   num_utils::minimax_f32,
   string_utils::quot,
   types::{stamp,Stamped}
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
pub type Token = String;
pub type RawPrices = HashMap<TokenId, Quote>;
pub type Dict = HashMap<TokenId, Token>;

pub type Pivots = Vec<String>;

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
   base: String,
   target: String
}

fn mk_name(t1: &str, t2: &str) -> Name {
   Name { base: t2.to_string(), target: t1.to_string() }
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

pub fn mk_ratios(t1: &str, t2: &str,
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

pub fn mk_emas(t1: &str, t2: &str, period: usize,
               dates: &Vec<NaiveDate>,
               ratios: &Vec<f32>) -> EMAs {

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
            println!("Confidence for {} trade: {:.2?}%", stamped_delta.d.date,
                     conf * 100.0);
            Some(conf)
         })
      })
   }).or_else(||{ println!("no confidence"); None })
}

// ----- Chart-data, or, fetching historical data for tokens -----------------

pub type StampedData<A> = HashMap<NaiveDate, A>;
pub type Chart<A> = HashMap<String, StampedData<A>>;

// reading functionality in fetch_prices and (eventually) snarf
