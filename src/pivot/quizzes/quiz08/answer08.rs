use std::iter::zip;

use book::{
   err_utils::ErrStr,
   utils::get_args
};

use swerve::{
   fetch_pivots::{fetch_lines,parse_token_headers,parse_row},
   reports::portfolio_prices,
   types::{Price,Quote,TokenId,Token}
};

fn usage() {
   println!("\n./stat");
   println!("\tSnarfs pivots.csv and reports the latest numbers.\n");
   println!("...allow 5 minutes after pivots.csv updated for raw to catch up.");
}

#[tokio::main]
async fn main() -> ErrStr<()> {
   let args = get_args();
   if args.first().is_some() { usage(); }
   let pivs = fetch_lines().await?;
   let header = parse_token_headers(&pivs);
   if let Some(line) = pivs.last() {
      let (date, nums) = parse_row(&line)?;
      portfolio_prices(&date, &to_portfolio(header, nums));
      Ok(())
   } else {
      Err("No last pivot!".to_string())
   }
}

fn to_portfolio(hdr: Vec<(TokenId,Token)>, prices: Vec<Quote>) -> Vec<Price> {
   zip(hdr.into_iter(), prices.into_iter()).collect()
}
