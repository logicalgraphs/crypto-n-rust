use book::utils::get_args;

use crypto::{
   algos::prices::{merge_prices,print_sorted_prices},
   parsers::token_prices::read_prices,
   types::prices::prices_with_aliases
};

fn main() {
   if let [date, portfolio] = get_args().as_slice() {
      let port = read_prices(&portfolio);
      let news = prices_with_aliases(&date);
      print_sorted_prices(&merge_prices(&news,&port));
   } else {
      usage();
   }
}

fn usage() {
   println!("./kfin <date> <prices.tsv>\n");
   println!("\tComputes the actively traded token prices on Kujira FIN,");
   println!("\tusing <prices.tsv> as the schema on which tokens to report.");
}
