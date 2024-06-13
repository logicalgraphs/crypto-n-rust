// name-explanation: when you've got to fetch-all, you're snarfin' it!
// ... am I right, fam? 😎

use book::err_utils::ErrStr;

use crate::{
   fetch_pivots::{Pivots,fetch_pivots},
   token_ids::{Dict,extract_keys_symbols}
};

pub async fn snarf_pivots() -> ErrStr<(Pivots, Dict)> {
   let pivs = fetch_pivots().await?;
   let dict = extract_keys_symbols(&pivs);
   Ok((pivs, dict))
}
