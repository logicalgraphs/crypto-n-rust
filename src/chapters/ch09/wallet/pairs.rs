use std::fmt;

use book::csv_utils::CsvWriter;
use crypto::types::usd::{USD,no_monay};

#[derive(Debug, Clone)]
pub struct Pair<T> {
   pub k: String,
   pub v: T
}

pub fn mk_pair<T: Clone>(key: &str, val: T) -> Pair<T> {
   Pair { k: key.to_string(), v: val.clone() }
}

impl Default for Pair<USD> {
   fn default() -> Self {
      mk_pair("_", no_monay())
   }
}

impl<T: fmt::Display> CsvWriter for Pair<T> {
   fn as_csv(&self) -> String { format!("{},{}", self.k, self.v) }
   fn ncols(&self) -> usize { 2 }
}
