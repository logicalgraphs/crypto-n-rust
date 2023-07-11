use book::{
   csv_utils::CsvWriter,
   json_utils::unquot
};

extern crate serde;

use serde::{Deserialize,Deserializer};
use serde_json::{Value, from_str, Value::Bool};

#[derive(Deserialize)]
pub struct LSDs {
   #[serde(rename(deserialize="host_zone"))]
   lsds: Vec<LSD>
}

#[derive(Debug,Clone)]
pub struct LSD {
   zone: String,
   base: String,
   rate: f32,
   unbond: u8,
   halted: bool
}

pub fn token(lsd: &LSD) -> String {
   let (frist, sym) = lsd.base.split_at(1);
   let up_sym = if "au".contains(frist) { sym.to_string()
                } else { format!("{frist}{sym}") }.to_uppercase();
   format!("st{up_sym}")
}

impl CsvWriter for LSD {
   fn as_csv(&self) -> String {
      format!("{},{},{:.4},{},{}",
              self.zone, token(self), self.rate, self.unbond, self.halted)
   }
}

pub fn print_lsds(date: &str, lsds: &Vec<LSD>) {
   println!("date,zone,lsd,exchange,burn,halted");
   for lsd in lsds {
      if !lsd.halted {
         println!("{date},{}", lsd.as_csv());
      }
   }
}

pub fn parse_lsds(str: &str) -> Vec<LSD> {
   let lsds: LSDs = from_str(str).expect("Where'd the JSON go???");
   lsds.lsds
}

impl<'de> Deserialize<'de> for LSD {
   fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
         where D: Deserializer<'de> {
      let json: Value = Value::deserialize(deserializer)?;
      let zone = unquot(&json, "bech32prefix");
      let base = unquot(&json, "host_denom");
      let rate1 = unquot(&json, "redemption_rate");
      let rate: f32 = rate1.parse().expect("redemption_rate");
      let unbond1 = unquot(&json, "unbonding_frequency");
      let unbond: u8 = unbond1.parse().expect("unbonding_frequency");
      let raw = &json["halted"];
      if let Bool(halted) = *raw {
         Ok(LSD { zone, base, rate, unbond, halted })
      } else { panic!("{raw} is not bool") }
   }
}
