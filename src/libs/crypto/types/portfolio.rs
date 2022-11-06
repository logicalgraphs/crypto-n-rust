// Portfolio isn't a 'type', per se, but we treat the portfolio typefully.
// ... typefulness? ... typetude?

// yeah, that.

use std::{
   collections::HashSet,
   path::Path
};

use crate::types::{
   assets::{Asset,print_assets,read_assets},
   trades::{Swap,swap_d},
   usd::USD
};

use book::file_utils::lines_from_file;

#[derive(Debug, Clone)]
pub struct Portfolio {
   bag: HashSet<Asset>
}

// creators

pub fn portfolio() -> Portfolio {
   let bag = HashSet::new();
   Portfolio { bag }
}

pub fn seed_portfolio(bag: HashSet<Asset>) -> Portfolio {
   let bag1 = bag.clone();
   Portfolio { bag: bag1 }
}

pub fn assets_from_file(file: impl AsRef<Path>) -> Portfolio {
   let lines = lines_from_file(file);
   let (_, daters) = lines.split_at(3);
   Portfolio { bag: read_assets(daters.to_vec()) }
}

// -- and then

pub fn execute(p: &Portfolio, trade: Swap) -> (Portfolio, USD) {
   execute_d(p, trade, false)
}

pub fn execute_d(p: &Portfolio, trade: Swap, debug: bool) -> (Portfolio, USD) {
   let mut b = p.bag.clone();
   let (b1, pnl) = swap_d(&mut b, trade, debug);
   (Portfolio { bag: b1 }, pnl)
}

pub fn print_portfolio(p: &Portfolio) {
   println!("\nPortfolio:\n");
   print_assets(&p.bag);
}
