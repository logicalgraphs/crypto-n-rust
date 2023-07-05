// we read in the marketplace and write out a csv WHICH WE GRAPH!

use book::utils::get_args;

use crypto::{
   algos::orders::read_marketplace,
   types::marketplace::{print_marketplace,prices,merge_synthetics}
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
      merge_synthetics(&mut market, &quotes, synthetics);
      print_marketplace(&market);
   } else {
      usage();
   }
}
