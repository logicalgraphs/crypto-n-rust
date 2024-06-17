use std::collections::HashMap;

use book::csv_utils::CsvWriter;

extern crate serde;

use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct Quote { usd: f32 }

impl CsvWriter for Quote {
   fn as_csv(&self) -> String { format!("{}", self.usd) }
   fn ncols(&self) -> usize { 1 }
}

pub type TokenId = String;
pub type Token = String;
pub type Dict = HashMap<TokenId, Token>;

pub type Pivots = Vec<String>;

pub type Price = ((TokenId, Token), Quote);
