use std::collections::HashMap;

use book::num_utils::parse_commaless;

use crypto::types::usd::{USD,mk_usd};

use crate::{
   pairs::{Pair,mk_pair},
   tsv::TsvWriter
};

#[derive(Debug, Clone)]
pub struct Token {
   token: String,
   amount: f32
}
   
impl TsvWriter for Token {
   fn as_tsv(&self) -> String { format!("{}\t{}", self.token, self.amount) }
}  

fn scan_token(tok: &str, whole: f32, fract: f32) -> Token {
   let num = format!("{whole}.{fract}");
   let amt: Result<f32, _> = num.parse();
   if let Ok(amount) = amt {
      let token = tok.to_string();
      Token { token, amount }
   } else {
      panic!("Cannot convert {num} for {tok}")
   }
}
   
impl Default for Token {
   fn default() -> Self {
      scan_token("", 0.0, 0.0)
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
         if let Ok(fract) = parse_commaless(&window[2]) {
            return Some((idx, scan_token(&window[0], whole, fract)))
         }
      }
   }
   None
}

pub fn token_pair(t: &Token) -> Pair<f32> { mk_pair(&t.token, t.amount) }
