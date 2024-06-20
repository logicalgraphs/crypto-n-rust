use std::collections::HashMap;

use book::csv_utils::CsvWriter;

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

#[derive(PartialEq)]
pub enum Diff { MISSING, ADDED }

impl CsvWriter for Diff {
   fn as_csv(&self) -> String {
      (if self == &Diff::MISSING { "missing" } else { "added"}).to_string()
   }
   fn ncols(&self) -> usize { 1 }
}

pub type Diffs = (Diff, Vec<String>);
