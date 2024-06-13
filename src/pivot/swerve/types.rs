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

pub type Dict = HashMap<String, String>;

pub type Pivots = Vec<String>;

pub type TokenId = String;
pub type Token = String;
pub type Price = ((TokenId, Token), Quote);
