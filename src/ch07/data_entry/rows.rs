use std::{
   collections::HashSet,
   hash::{Hash,Hasher},
};

use book::{
   csv_utils::CsvWriter,
   list_utils::{head},
   num_utils::parse_commaless
};

use crypto::types::usd::USD;

#[derive(Debug, Clone)]
pub struct Row {
   token: String,
   pub amount: f32,
   value: USD
}

pub fn mk_row(tok: &str, amount: f32, value: USD) -> Row {
   Row { token: tok.to_string(), amount, value }
}

pub fn locate(key: &str, set: &HashSet<Row>) -> Option<Row> {
   for val in set {
      if val.token == key {
         return Some(val.clone())
      }
   }
   None
}

impl PartialEq for Row {
   fn eq(&self, other: &Self) -> bool {
      self.token == other.token
   }
}

impl Eq for Row { }

impl Hash for Row {
   fn hash<H: Hasher>(&self, state: &mut H) {
      self.token.hash(state);
      self.value.hash(state);
   }
}

impl CsvWriter for Row {
   fn as_csv(&self) -> String {
      format!("{},{},{}", self.token, self.amount, self.value)
   }
}

pub fn find_triple(body: &Vec<&String>) -> Option<(usize,Row)> {
   for (idx,window) in body.windows(3).enumerate() {
      if let Some(amount) = is_num(window[1]) {
         if let Some(value) = is_monay(window[2]) {
            return Some((idx, mk_row(window[0], amount, value)))
         }
      }
   }
   None
}

fn is_num(line: &str) -> Option<f32> {
   let positions: Vec<&str> = line.split(' ').collect();
   if let Some(position) = head(&positions) {
      match parse_commaless(&position.to_string()) {
         Ok(res) => Some(res),
         Err(_)  => None
      }
   } else { None }
}

fn is_monay(line: &str) -> Option<USD> {
   match line.parse() {
      Ok(ans) => Some(ans),
      Err(_) => None
   }
}
