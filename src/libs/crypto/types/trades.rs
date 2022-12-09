// a trade is a swap from one asset to another

use std::collections::HashSet;

use book::csv_utils::{CsvWriter,print_csv};

use crate::types::{
   assets::{Asset,parse_asset,add_asset,remove_asset,print_asset_d,diff_usd},
   usd::{mk_usd,USD}
};

#[derive(Debug, Clone)]
pub struct Swap {
   pub date: String, // because date is relevant, how?
   from: Asset,
   to: Asset,
   fees: USD,
   commission: USD
}

// ----- impls -------------------------------------------------------

impl CsvWriter for Swap {
   fn as_csv(&self) -> String {
      format_args!("{},{},{},{},{}",self.date, 
                   self.from.as_csv(), self.to.as_csv(),
                   self.fees, self.commission)
         .to_string()
   }
}

// ---- first task is to parse in orders ----------------------------

pub fn mk_swap(date: String, from: Asset, to: Asset, fees: USD, commission: USD)
   -> Swap {
   Swap { date, from, to, fees, commission }
}

pub fn parse_swap(date: &str, sym1: &str, amt1: &str, sym2: &str, amt2: &str,
                  quot2: &str, quot1: &str, costs: Vec<&str>)
    -> Result<Swap, String> {
   let to     = parse_asset(sym1, amt1, quot1)?;
   let from   = parse_asset(sym2, amt2, quot2)?;
   let costs1: Vec<&&str> = costs.iter().rev().collect();
   let (rev_costs, _) = costs1.split_at(3);
   if let [comm, _, fee] = rev_costs {
      let commission: USD = comm.parse().expect("commission");
      let fees: USD = fee.parse().expect("fees");
      Ok(mk_swap(date.to_string(), from, to, fees, commission))
   } else {
      Err("Could not collect fee and commission data".to_string())
   }
}

pub fn read_csv_swap(line: &String) -> Result<Swap, String> {
   let mut swap_dater: Vec<&str> = line.split(',').collect();
   swap_dater.truncate(15);
   let (swap, fees) = swap_dater.split_at(8);
   if let [_, dat, sym1, amt1, sym2, amt2, qut1, qut2] = swap {
      parse_swap(dat, sym1, amt1, sym2, amt2, qut1, qut2, fees.to_vec())
   } else {
      Err("Can't parse line: ".to_owned() + line)
   }
}

// now let's execute the swap against a (hash)set of assets.

pub fn swap(p: &mut HashSet<Asset>, s: &Swap) -> (HashSet<Asset>, USD) {
   swap_d(p, s, false)
}

pub fn swap_d(p: &mut HashSet<Asset>, s: &Swap, debug: bool)
   -> (HashSet<Asset>, USD) {
   if debug {
      println!("For trade");
      print_csv(s);
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

pub fn expenses(s: &Swap) -> (USD, USD) {
   (s.fees, s.commission)
}

// computes the profit (or loss) on an asset sold vice what I had it as

pub fn pnl(bag: &HashSet<Asset>, sold: &Asset) -> USD {
   match bag.get(sold) {
      None => mk_usd(0.0),
      Some(orig) => diff_usd(orig, sold)
   }
}
