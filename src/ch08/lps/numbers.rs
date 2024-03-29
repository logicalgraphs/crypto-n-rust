/*

parses a number, n, that spans multiple lines. Also returns the type of that
number, be it a percentage or a dallah-dallah.

*/

use book::{
   list_utils::tail,
   num_utils::parse_commaless
};

use crypto::types::{
   usd::{mk_usd,USD}
};

use crate::percs::{Perc,mk_perc};

enum Measure {
   Dollar(USD),
   Percent(Perc)
}

fn parse_measure(lines: Vec<String>) -> Result<Measure, String> {
   if let [whole,fract,typ] = lines.as_slice() {
      let amount = format!("{whole}{fract}");
      if let Ok(bigly) = parse_commaless(&amount) {
         match typ.as_str() {
            "$" => Ok(Measure::Dollar(mk_usd(bigly))),
            "%" => Ok(Measure::Percent(mk_perc(bigly/100.0))),
            _   => Err(format!("Do not know the type: {typ}"))
         }
      } else {
         Err(format!("Could not parse amount {amount}"))
      }
   } else {
      Err("Not yet implemented".to_string())
   }
}

pub fn parse_usd(lines: &Vec<String>) -> (Result<USD, String>, Vec<String>) {
   let (parsley, sage) = lines.split_at(3);
   if let Ok(Measure::Dollar(ans)) = parse_measure(parsley.to_vec()) {
      (Ok(ans), sage.to_vec())
   } else {
      (Err(format!("Could not parse dollar from {parsley:?}")), lines.to_vec())
   }
}

pub fn parse_percent(lines: &Vec<String>)
   -> (Result<Perc, String>, Vec<String>) {
   let (parsley, sage) = lines.split_at(3);
   if let Ok(Measure::Percent(ans)) = parse_measure(parsley.to_vec()) {
      (Ok(ans), sage.to_vec())
   } else {
      (Err(format!("Could not parse percent from {parsley:?}")), lines.to_vec())
   }
}

pub fn parse_percent_or_collecting(lines: &Vec<String>)
   -> (Perc, Vec<String>) {
   let (val, rest) = parse_percent(lines);
   match val {
     Err(_) => (mk_perc(0.0), tail(&rest)),
     Ok(p) => (p, rest)
   }
}
