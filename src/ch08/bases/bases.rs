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

use crypto::{
   types::marketplace::{prices,prices_usk},
   algos::orders::read_marketplace
};


fn usage() {
   println!("\n./bases <marketplace JSON file>");
   println!("\n\tprints the marketplace tokens and prices");
}

fn main() {
   if let Some(filename) = head(get_args()) {
      println!("Processing {}", filename);
      let markets = read_marketplace(&filename);
      print_prices("Prices", &prices(&markets));

      // bonus:
      print_prices("USKs", &prices_usk(&markets));
   } else {
      usage();
   }
}

fn print_prices<T: Display>(header: &str, p: &HashMap<String, T>) {
   println!("\n{header}:\n");
   let mut v: Vec<_> = p.into_iter().collect();
   v.sort_by(|x,y| x.0.cmp(&y.0));
   v.iter().for_each(|(k,v)| println!("{k}  {v}"));
}
