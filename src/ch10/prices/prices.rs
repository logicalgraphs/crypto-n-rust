use book::utils::get_args;

use crypto::{
   algos::prices::print_sorted_prices,
   types::prices::prices_with_aliases
};

fn usage() {
   println!("./prices <date>\n");
   println!("Extract the prices of the actively traded crypto on Kujira FIN");
}

fn main() { 
   if let Some(date) = get_args().first() {
      print_sorted_prices(&prices_with_aliases(date));
   } else {
      usage();
   }
}
