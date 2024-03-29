use crypto::{
   algos::prices::print_sorted_prices,
   types::prices::prices_with_aliases
};

fn main() { print_sorted_prices(&prices_with_aliases()); }

// ... welp.
