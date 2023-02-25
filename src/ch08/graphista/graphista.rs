// we read in the marketplace and write out a csv WHICH WE GRAPH!

use book::utils::get_args;

use crypto::types::marketplace::{
   read_marketplace,print_marketplace,prices,read_synthetic_order_books
};

fn usage() {
   let m = "<market LSV file>";
   let s = "<synthetics TSV file>";
   println!("./graphista {m} {s} > data/market-graph.csv");
   println!("\n\treads in the market data and outputs it as CSV TO GRAPH!");
}

fn main() {
   if let [marketplace, synthetics] = get_args().as_slice() {
      let mut market = read_marketplace(&marketplace);
      let quotes = prices(&market);
      let synths = read_synthetic_order_books(&synthetics, &quotes);
      for s in synths {
         market.insert(s.clone());
      }
      print_marketplace(&market);
   } else {
      usage();
   }
}
