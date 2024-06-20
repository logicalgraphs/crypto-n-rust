// name-explanation: when you've got to fetch-all, you're snarfin' it!
// ... am I right, fam? 😎

use std::env::var;

use book::err_utils::{err_or,ErrStr};

use crate::{
   types::{Dict,Diffs,Pivots,Price},
   fetch_pivots::{fetch_lines,parse_keys_symbols},
   fetch_prices::{fetch_prices,transform_prices},
   verify::verify
};

pub async fn snarf_pivots() -> ErrStr<(Pivots, Dict)> {
   let pivs = fetch_lines().await?;
   let dict = parse_keys_symbols(&pivs);
   Ok((pivs, dict))
}

// the el biggie en-snarf-ifier!

pub async fn snarf() -> ErrStr<(Vec<Price>, Option<Diffs>)> {
   let (_pivs, dict) = snarf_pivots().await?;
   let pass = err_or(var("COIN_GECKO_API_KEY"),
                     "Could not fetch API key from environment")?;
   let raw_prices = fetch_prices(&pass, &dict).await?;
   let errs = verify(&dict, &raw_prices);
   let prices = transform_prices(&dict, &raw_prices);
   // or, with arrows: (verify &&& transform_prices) (&dict, &raw_prices)
   Ok((prices, errs))
}
