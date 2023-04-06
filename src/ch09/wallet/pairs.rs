use std::fmt;

use crypto::types::usd::{USD,no_monay};

use crate::tsv::TsvWriter;

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

impl<T: fmt::Display> TsvWriter for Pair<T> {
   fn as_tsv(&self) -> String { format!("{}\t{}", self.k, self.v) }
}

