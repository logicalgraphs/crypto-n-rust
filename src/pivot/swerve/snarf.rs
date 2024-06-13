// name-explanation: when you've got to fetch-all, you're snarfin' it!
// ... am I right, fam? 😎

use std::env::var;

use book::err_utils::ErrStr;

use crate::{
   types::{Dict,Pivots},
   fetch_pivots::fetch_pivots
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

pub async fn snarf() -> ErrStr<(
