use std::collections::HashMap;

use book::{
   csv_utils::CsvWriter,
   num_utils::parse_commaless
};

use crypto::types::usd::{USD,mk_usd};

use crate::pairs::{Pair,mk_pair};

#[derive(Debug, Clone)]
pub struct Token {
   token: String,
   amount: f32
}

pub fn is_xtoken(t: &Token) -> bool {
   &t.token == "$"
}
   
impl CsvWriter for Token {
   fn as_csv(&self) -> String { format!("{},{}", self.token, self.amount) }
   fn ncols(&self) -> usize { 2 }
}  

fn mk_token(tok: &str, amount: f32) -> Token {
   let token = tok.to_string();
   Token { token, amount }
}
   
impl Default for Token {
   fn default() -> Self {
      mk_token("", 0.0)
   }
}

pub fn token_value(m: &HashMap<String, USD>)
    -> impl Fn(&Token) -> Option<Pair<USD>> + '_ {
   |t| {
      let namei = &t.token;
      if let Some(price) = m.get(namei) {
         Some(mk_pair(namei, mk_usd(price.amount * t.amount)))
      } else {
         None
      }
   }
}

pub fn find_token(lines: &Vec<String>) -> Option<(usize, Token)> {
   for (idx, window) in lines.windows(3).enumerate() {
      if let Ok(whole) = parse_commaless(&window[1]) {
         if let Ok(fract) = parse_commaless(&format!("0.{}", &window[2])) {
            return Some((idx, mk_token(&window[0], whole+fract)))
         }
      }
   }
   None
}

pub fn token_pair(t: &Token) -> Pair<f32> { mk_pair(&t.token, t.amount) }
