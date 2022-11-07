// gets the axlUSDC prices of all assets, including the ones that aren't
// paired with axlUSDC. How do we do this? Magic. PFM.

use book::{
   utils::get_args,
   list_utils::head
};

use crypto::types::marketplace::{read_marketplace,prices};

fn usage() {
   println!("\n./bases <marketplace LSV file>");
   println!("\n\tprints the marketplace tokens and prices");
}

fn main() {
   if let Some(filename) = head(get_args()) {
      println!("Processing {}", filename);
      let markets = read_marketplace(filename);
      println!("Prices:\n");
      for (k, v) in prices(&markets) {
         println!("{k}  {v}");
      }
   } else {
      usage();
   }
}
