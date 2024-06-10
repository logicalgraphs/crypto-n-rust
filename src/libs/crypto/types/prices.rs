// extracts current market data from 
// https://api.kujira.app/api/coingecko/tickers

use crate::{
   rest_utils::fin_res,
   types::{
      aliases::load_aliases,
      internal::{
         prices::prices_from_books,
         types::raw_books
      },
      interfaces::Prices
   }
};

pub fn prices_with_aliases(date: &str) -> Prices {
   prices(&date, Some(fin_res("main", "aliases.csv")))
}

pub fn prices(date: &str, opt_aliases: Option<String>) -> Prices {
   let b0 = raw_books();
   let aliases = load_aliases(&opt_aliases);
   prices_from_books(&date, &b0, &aliases)
}
