use std::env::var;

use book::err_utils::{err_or,ErrStr};

use swerve::{
   fetch_prices::fetch_prices,
   read_rest::read_pivots,
   token_ids::extract_keys_symbols
};

#[tokio::main]
async fn main() -> ErrStr<()> {
   let pass = err_or(var("COIN_GECKO_API_KEY"),
                     "Could not fetch API key from environment")?;
   let pivs = read_pivots().await?;
   let toks = extract_keys_symbols(&pivs);
   let ids: Vec<String> = toks.keys().take(3).map(String::to_string).collect();
   let prices = fetch_prices(&pass, &ids).await?;
   println!("I got for prices: {prices}");
   Ok(())
}
