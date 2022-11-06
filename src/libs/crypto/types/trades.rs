// a trade is a swap from one asset to another

use std::collections::HashSet;

use book::csv_utils::{CsvWriter,print_csv};

use crate::types::{
   assets::{Asset,parse_asset,add_asset,remove_asset,print_asset_d,diff_usd},
   usd::{mk_usd,USD}
};

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

// ---- first task is to parse in orders ----------------------------

pub fn mk_swap(date: String, from: Asset, to: Asset) -> Swap {
   Swap { date, from, to }
}

pub fn parse_swap(date: &str, sym1: &str, amt1: &str, sym2: &str, amt2: &str,
                   quot2: &str, quot1: &str) -> Result<Swap, String> {
   let to     = parse_asset(sym1, amt1, quot1)?;
   let from   = parse_asset(sym2, amt2, quot2)?;
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

// now let's execute the swap against a (hash)set of assets.

pub fn swap(p: &mut HashSet<Asset>, s: Swap) -> (HashSet<Asset>, USD) {
   swap_d(p, s, false)
}

pub fn swap_d(p: &mut HashSet<Asset>, s: Swap, debug: bool)
   -> (HashSet<Asset>, USD) {
   if debug {
      println!("For trade");
      print_csv(&s);
   }
   let tom = s.to.clone();
   add_asset(p, s.to.clone());
   print_asset_d(&p, &tom, debug);
   let fromm = s.from.clone();
   let zuppa = pnl(p, &fromm);
   let bag = remove_asset(p, s.from.clone());
   print_asset_d(&bag, &fromm, debug);
   if debug { println!("PnL: {}", zuppa); }
   (bag, zuppa)
}

// computes the profit (or loss) on an asset sold vice what I had it as

pub fn pnl(bag: &HashSet<Asset>, sold: &Asset) -> USD {
   match bag.get(sold) {
      None => mk_usd(0.0),
      Some(orig) => diff_usd(orig, sold)
   }
}
