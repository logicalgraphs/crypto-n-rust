// gets the axlUSDC prices of all assets, including the ones that aren't
// paired with axlUSDC. How do we do this? Magic. PFM.

use std::{
   collections::HashMap,
   fmt::Display
};

use book::{
   utils::get_args,
   list_utils::head
};

use crypto::types::marketplace::{read_marketplace,prices,prices_usk};

fn usage() {
   println!("\n./bases <marketplace LSV file>");
   println!("\n\tprints the marketplace tokens and prices");
}

fn main() {
   if let Some(filename) = head(get_args()) {
      println!("Processing {}", filename);
      let markets = read_marketplace(filename);
      print_prices("Prices", &prices(&markets));

      // bonus:
      print_prices("USKs", &prices_usk(&markets));
   } else {
      usage();
   }
}

fn print_prices<T: Display>(header: &str, p: &HashMap<String, T>) {
   println!("\n{header}:\n");
   for (k, v) in p {
      println!("{k}  {v}");
   }
}
