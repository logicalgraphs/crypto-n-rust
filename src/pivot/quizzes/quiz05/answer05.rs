use std::env::var;

use book::err_utils::ErrStr;

use swerve::{
   read_rest::read_pivots,
   token_ids::extract_ids_symbols
};

fn main() -> ErrStr<()> {
   let pass = var("COIN_GECKO_API_KEY")?;
   let pivs = read_pivots()?;
   let toks = extract_ids_symbols(&pivs);
}
