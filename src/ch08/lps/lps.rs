// extract info about the FIN liquidity pools then reports on these data.

use std::fmt;

use book::{
   file_utils::extract_date_and_body,
   list_utils::ht,
   utils::get_args
};

use crypto::types::{
   percentage::Percentage,
   usd::USD
};

mod numbers;
use crate::numbers::{parse_usd,parse_percent,skip_percent_or_collecting};

#[derive(Debug,Clone)]
struct LP {
   name: String,
   volume: USD,
   apr: Percentage
}

impl fmt::Display for LP {
   fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
      write!(formatter, "LP {} volume: {}, APR: {}",
             self.name, self.volume, self.apr)
   }
}

fn main() {
   let files = get_args();
   for file in files {
      let (_, lines) = extract_date_and_body(file);
      process_lps(lines);
   }
   println!("Hi, mom!");
}

fn process_lps(lines: Vec<String>) {
   let mut lps: Vec<LP> = Vec::new();
   process_lp(lines, &mut lps);
   lps.iter().for_each(print_lp);
}

fn process_lp(lines: Vec<String>, lps: &mut Vec<LP>) {
   let meat: Vec<String> =
      lines.into_iter().skip_while(|x| !x.contains('/')).collect();
   if !meat.is_empty() {
      if let (Some(lp), rest) = ht(meat) {
         let (vol, rest1) = parse_usd(&rest);
         let rest2 = skip_percent_or_collecting(&rest1);
         let (aprr, rest3) = parse_percent(&rest2);
         if let Ok(volume) = vol {
            if let Ok(apr) = aprr {
               lps.push(LP { name: lp, volume, apr });
            }
         }
         process_lp(rest3, lps);
      }
   }
}

fn print_lp(lp: &LP) {
   println!("I got {lp}");
}
