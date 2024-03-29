// extracts current market data from 
// https://api.kujira.app/api/coingecko/tickers

use crate::{
   rest_utils::graphs_fin_res,
   types::{
      aliases::load_aliases,
      internal::{
         books::raw_books,
         prices::prices_from_books
      },
      interfaces::Prices
   }
};

pub fn prices_with_aliases(date: &str) -> Prices {
   prices(&date, Some(graphs_fin_res("aliases.csv")))
}

pub fn prices(date: &str, opt_aliases: Option<String>) -> Prices {
   let b0 = raw_books();
   let aliases = load_aliases(&opt_aliases);
   prices_from_books(&date, &b0, &aliases)
}
