// a trade is a swap from one asset to another

use std::collections::HashSet;

use book::{
   csv_utils::{CsvWriter,print_csv},
   list_utils::tail
};

use crate::types::{
   assets::{Asset,parse_asset,add_asset,remove_asset,print_asset_d,diff_usd},
   liquidations::{Liquidation,gather_liquidation_info},
   percentage::{Percentage,mk_percentage},
   usd::{mk_usd,USD}
};

#[derive(Debug, Clone)]
pub struct Swap {
   pub date: String, // because date is relevant, how?
   from: Asset,
   to: Asset,
   fees: USD,
   commission: USD,
   liquidation: Option<Liquidation>
}

// ----- impls -------------------------------------------------------

impl CsvWriter for Swap {
   fn as_csv(&self) -> String {
      format_args!("{},{},{},{},{},{}",self.date, 
                   self.from.as_csv(), self.to.as_csv(),
                   self.fees, self.commission,
                   self.liquidation.as_ref()
                      .map_or(String::new(), |l| format!("{}", l.percentage)))
         .to_string()
   }
}

// ---- first task is to parse in orders ----------------------------

pub fn mk_swap(date: String, from: Asset, to: Asset, fees: USD, 
               commission: USD, liquidation: Option<Liquidation>) -> Swap {
   Swap { date, from, to, fees, commission, liquidation }
}

pub fn parse_swap(date: &str, sym1: &str, amt1: &str, sym2: &str, amt2: &str,
                  quot2: &str, quot1: &str, perc: Option<Percentage>,
                  costs: Vec<&str>) -> Result<Swap, String> {
   let to     = parse_asset(sym1, amt1, quot1)?;
   let from   = parse_asset(sym2, amt2, quot2)?;
   let costs1: Vec<&&str> = costs.iter().skip(4).take(2).collect();
   if let [fee, comm] = costs1.as_slice() {
      let commission: USD = comm.parse().expect("commission");
      let fees: USD = fee.parse().expect("fees");
      let liq = perc.map(|p| to.liquidated_at(p));
      Ok(mk_swap(date.to_string(), from, to, fees, commission, liq))
   } else {
      Err("Could not collect fee and commission data".to_string())
   }
}

pub fn read_csv_swap(line: &String) -> Result<Swap, String> {
   let mut swap_dater: Vec<&str> = line.split(',').collect();
   swap_dater.pop();
   let mut daters = tail(swap_dater);
   let perc = gather_liquidation_info(&mut daters)?;
   let t_daters = tail(daters);
   let (swap, fees) = t_daters.split_at(7);
   if let [dat, sym1, amt1, sym2, amt2, qut1, qut2] = swap {
      parse_swap(dat, sym1, amt1, sym2, amt2, qut1, qut2, perc, fees.to_vec())
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
      println!("\nFor trade");
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
      None => { println!("Can't find {sold:?}"); mk_usd(0.0) },
      Some(orig) => diff_usd(orig, sold)
   }
}

// Are we a liquidation?

pub fn liquidations_count_and_premium(trades: &Vec<Swap>) -> (u8, Percentage) {
   let (count, weight, sum) =
      trades.iter().fold((0, 0.0, 0.0), | (c, w, s), t | {
      if let Some(liq) = &t.liquidation {
         (c + 1, w + liq.weight(), s + liq.amount.amount)
      } else {
         (c, w, s)
      }
   });
   (count, mk_percentage(weight / sum))
}
