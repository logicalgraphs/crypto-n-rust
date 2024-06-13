// name-explanation: when you've got to fetch-all, you're snarfin' it!
// ... am I right, fam? 😎

use std::{
   env::var,
   iter::zip
};

use book::{
   err_utils::{err_or,ErrStr},
   string_utils::to_string
};

use crate::{
   types::{Dict,Pivots,Price},
   fetch_pivots::fetch_pivots,
   fetch_prices::fetch_prices
};

fn parse_keys_symbols(pivots: &Pivots) -> Dict {
   let ids = pivots[0].split(",").skip(1).map(to_string);
   let syms = pivots[1].split(",").skip(1).map(to_string);
   zip(ids, syms).collect()
}

pub async fn snarf_pivots() -> ErrStr<(Pivots, Dict)> {
   let pivs = fetch_pivots().await?;
   let dict = parse_keys_symbols(&pivs);
   Ok((pivs, dict))
}

// the el biggie en-snarf-ifier!

pub async fn snarf() -> ErrStr<Vec<Price>> {
   let (_pivs, dict) = snarf_pivots().await?;
   let pass = err_or(var("COIN_GECKO_API_KEY"),
                     "Could not fetch API key from environment")?;
   let prices = fetch_prices(&pass, &dict).await?;
   Ok(prices)
}
