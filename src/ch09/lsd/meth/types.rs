use std::{
   cmp::Ordering,
   collections::HashMap
};

use book::{
   csv_utils::CsvWriter,
   json_utils::unquot
};

extern crate serde;

use serde::{Deserialize,Deserializer};
use serde_json::{Value, from_str, Value::Bool};

#[derive(Deserialize)]
pub struct BurnlessLSDs {
   #[serde(rename(deserialize="host_zone"))]
   lsds: Vec<BurnlessLSD>
}

#[derive(Debug,Clone)]
pub struct BurnlessLSD {
   zone: String,
   base: String,
   rate: f32,
   halted: bool
}

#[derive(Debug,Clone,PartialEq)]
pub struct ManualLSD {
   zone: String,
   lsd: String,
   rate: f32,
   halted: bool,
   unbond: u8
}

#[derive(Debug,Clone)]
pub struct LSD {
   burnless: BurnlessLSD,
   pub unbond: u8
}

pub fn mk_fake_lsd(l: &BurnlessLSD) -> LSD {
   LSD { burnless: l.clone(), unbond: 0 }
}

fn token1(lsd: &BurnlessLSD) -> String {
   let (frist, sym) = lsd.base.split_at(1);
   let up_sym = if "au".contains(frist) { sym.to_string()
                } else { format!("{frist}{sym}") }.to_uppercase();
   format!("st{up_sym}")
}

pub fn token(lsd: &LSD) -> String { token1(&lsd.burnless) }

pub fn exchange_rate(lsd: &LSD) -> f32 {
   lsd.burnless.rate
}

pub fn merge_burn_rates(burnlesses: &Vec<BurnlessLSD>,
                        burns: &HashMap<String, u8>) -> Vec<LSD> {
   merge_burn_rates_d(burnlesses, burns, false)
}

pub fn merge_burn_rates_d(burnlesses: &Vec<BurnlessLSD>,
                          burns: &HashMap<String, u8>, debug: bool)
   -> Vec<LSD> {
   let mut lsds: Vec<LSD> = Vec::new();
   for b in burnlesses {
      let tok = token1(&b);
      if let Some(u) = burns.get(&tok) {
         lsds.push(LSD { burnless: b.clone(), unbond: *u });
      } else {
         if debug {
            println!("Could not find burn rate for {tok}");
            println!("for {b:?}");
            println!("I have:");
            for key in burns.keys() { println!("key: {key}"); }
            println!("\n...reminder to push burn-rates.csv to main branch.");
         }
      }
   }
   lsds
}  

// ----- Sorting ---------------------------------------------

impl PartialOrd for BurnlessLSD {
   fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
      Some(token1(self).cmp(&token1(other)))
   }
}

impl PartialOrd for LSD {
   fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
      Some(self.burnless.cmp(&other.burnless))
   }
}

impl PartialOrd for ManualLSD {
   fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
      Some(self.lsd.cmp(&other.lsd))
   }
}

impl Ord for BurnlessLSD {
   fn cmp(&self, other: &Self) -> Ordering {
      token1(self).cmp(&token1(other))
   }
}

impl Ord for LSD {
   fn cmp(&self, other: &Self) -> Ordering {
      self.burnless.cmp(&other.burnless)
   }
}

impl PartialEq for BurnlessLSD {
   fn eq(&self, other: &Self) -> bool {
      token1(self) == token1(other)
   }
}

impl Eq for BurnlessLSD {}

impl PartialEq for LSD {
   fn eq(&self, other: &Self) -> bool {
      self.burnless == other.burnless
   }
}

impl Eq for LSD {}

// ----- Printables ---------------------------------------------

impl CsvWriter for BurnlessLSD {
   fn as_csv(&self) -> String {
      format!("{},{},{:.4},{}", self.zone, token1(self), self.rate, self.halted)
   }
   fn ncols(&self) -> usize { 4 }
}

impl CsvWriter for LSD {
   fn as_csv(&self) -> String {
      format!("{},{}", self.burnless.as_csv(), self.unbond)
   }
   fn ncols(&self) -> usize { 2 }
}

pub fn print_lsds(date: &str, lsds: &Vec<LSD>) {
   println!("date,zone,lsd,exchange rate,halted,unbond (days)");
   let mut quarters: Vec<LSD> = lsds.clone();
   quarters.sort();
   quarters.into_iter().for_each(|lsd| {
      println!("{date},{}", lsd.as_csv());
   });
}

// ----- Parseables ---------------------------------------------

pub fn parse_lsds_without_burn(str: &str) -> Vec<BurnlessLSD> {
   let lsds: BurnlessLSDs = from_str(str).expect("Where'd the JSON go???");
   lsds.lsds
}

impl<'de> Deserialize<'de> for BurnlessLSD {
   fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
         where D: Deserializer<'de> {
      let json: Value = Value::deserialize(deserializer)?;
      let zone = unquot(&json, "bech32prefix");
      let base = unquot(&json, "host_denom");
      let rate1 = unquot(&json, "redemption_rate");
      let rate: f32 = rate1.parse().expect("redemption_rate");
      let raw = &json["halted"];
      if let Bool(halted) = *raw {
         Ok(BurnlessLSD { zone, base, rate, halted })
      } else { panic!("{raw} is not bool") }
   }
}
