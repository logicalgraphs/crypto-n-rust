// we read in the marketplace and write out a csv WHICH WE GRAPH!

use book::{
   list_utils::head,
   utils::get_args
};

use crypto::types::marketplace::{read_marketplace,print_marketplace};

fn usage() {
   println!("./graphista <market LSV file>");
   println!("\n\treads in the market data and outputs it as CSV TO GRAPH!");
}

fn main() {
   if let Some(file) = head(get_args()) {
      let market = read_marketplace(file);
      print_marketplace(&market);
   } else {
      usage();
   }
}
