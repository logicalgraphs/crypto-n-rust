use std::collections::HashMap;

use chrono::NaiveDate;

use book::{
   csv_utils::CsvWriter,
   json_utils::{AsJSON,json_list,to_object},
   string_utils::quot
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

#[derive(PartialEq,Debug,Clone)]
pub enum Diff { MISSING, ADDED }

impl CsvWriter for Diff {
   fn as_csv(&self) -> String {
      (if self == &Diff::MISSING { "missing" } else { "added"}).to_string()
   }
   fn ncols(&self) -> usize { 1 }
}

pub type Diffs = (Diff, Vec<String>);

// for EMA calculations

pub struct Ratio {
   dt: NaiveDate,
   ratio: f32
}

fn mk_ratio((dt, ratio): (&NaiveDate, &f32)) -> Ratio {
   Ratio { dt: dt.clone(), ratio: ratio.clone() }
}

impl AsJSON for Ratio {
   fn as_json(&self) -> String {
      to_object("date ratio",
                &[quot(&format!("{}", &self.dt)),
                  format!("{:?}", self.ratio)])
   }
}

pub struct Ratios {
   name: String,
   ratios: Vec<Ratio>
}

pub fn mk_ratios(t1: &str, t2: &str,
                 dates: &Vec<NaiveDate>, ratios: &Vec<f32>) -> Ratios {
   let dt_ratios: Vec<Ratio> =
      dates.into_iter().zip(ratios.into_iter()).map(mk_ratio).collect();
   Ratios { name: format!("{t1}/{t2}"), ratios: dt_ratios }
}

impl AsJSON for Ratios {
   fn as_json(&self) -> String {
      to_object("name ratios", &[quot(&self.name), json_list(&self.ratios)])
   }
}

pub struct EMAs {
   name: String,
   period: usize,
   emas: Vec<EMA>
}

pub struct EMA {
   dt: NaiveDate,
   ratio: f32,
   ema: f32
}

fn mk_ema((ratio, ema): (&Ratio, &f32)) -> EMA {
   EMA { dt: ratio.dt.clone(), ema: ema.clone(), ratio: ratio.ratio.clone() }
}

impl AsJSON for EMA {
   fn as_json(&self) -> String {
      to_object("date ratio ema",
                &[quot(&format!("{}", &self.dt)),
                  format!("{:?}", self.ratio),
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
                &[quot(&self.name), format!("{}", self.period),
                  json_list(&self.emas)])
   }
}
