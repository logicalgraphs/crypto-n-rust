// a trade is a swap from one asset to another

use book::csv_utils::CsvWriter;

use crate::types::assets::{Asset, parse_asset};

#[derive(Debug, Clone)]
pub struct Swap {
   date: String, // because date is relevant, how?
   from: Asset,
   to: Asset
}

// ----- impls -------------------------------------------------------

impl CsvWriter for Swap {
   fn as_csv(&self) -> String {
      format_args!("{},{},{}",self.date, self.from.as_csv(), self.to.as_csv())
         .to_string()
   }
}

// first task is to parse in orders

pub fn mk_swap(date: String, from: Asset, to: Asset) -> Swap {
   Swap { date, from, to }
}

pub fn parse_swap(date: &str, sym1: &str, amt1: &str, sym2: &str, amt2: &str,
                   quot1: &str, quot2: &str) -> Result<Swap, String> {
   let from = parse_asset(sym1, amt1, quot1)?;
   let to   = parse_asset(sym2, amt2, quot2)?;
   Ok(mk_swap(date.to_string(), from, to))
}

pub fn read_csv_swap(line: &String) -> Result<Swap, String> {
   let mut swap_dater: Vec<&str> = line.split(',').collect();
   swap_dater.truncate(7);
   if let [dat, sym1, amt1, sym2, amt2, quot1, quot2] = swap_dater.as_slice() {
      parse_swap(dat, sym1, amt1, sym2, amt2, quot1, quot2)
   } else {
      Err("Can't parse line: ".to_owned() + line)
   }
}