// name-explanation: when you've got to fetch-all, you're snarfin' it!
// ... am I right, fam? 😎

use std::{
   env::var,
   iter::{Iterator,zip}
};

use book::{
   compose,
   err_utils::{err_or,ErrStr},
   string_utils::to_string
};

use crate::{
   types::{Dict,Diffs,Pivots,Price},
   fetch_pivots::fetch_pivots,
   fetch_prices::{fetch_prices,transform_prices},
   verify::verify
};

fn parse_keys_symbols(pivots: &Pivots) -> Dict {

   fn splitter(line: &str) -> impl Iterator<Item=String> + '_ {
      line.split(",").skip(1).map(compose!(to_string)(str::trim_end))
   }
   let ids = splitter(&pivots[0]);
   let syms = splitter(&pivots[1]);
   zip(ids, syms).collect()
}

pub async fn snarf_pivots() -> ErrStr<(Pivots, Dict)> {
   let pivs = fetch_pivots().await?;
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
