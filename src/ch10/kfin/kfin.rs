use book::utils::get_args;

use crypto::{
   algos::prices::{merge_prices,print_sorted_prices},
   parsers::token_prices::read_prices,
   types::books::prices_with_aliases
};

fn main() {
   if let Some(portfolio) = get_args().first() {
      let port = read_prices(&portfolio);
      let news = prices_with_aliases();
      print_sorted_prices(&merge_prices(&news,&port));
   } else {
      usage();
   }
}

fn usage() {
   println!("./kfin <prices.csv>\n");
   println!("\tComputes the actively traded token prices on Kujira FIN,");
   println!("\tusing <prices.csv> as the schema on which tokens to report.");
}
