use std::iter::zip;

use clap::Parser;

use book::err_utils::ErrStr;

use swerve::{
   fetch_quotes::{fetch_lines,parse_token_headers,parse_row},
   reports::portfolio_prices,
   types::{Price,Quote,TokenId,Token}
};

/// Snarfs quotes.csv and reports the latest numbers.
///
/// ...allow 5 minutes after quotes.csv updated for raw to catch up.
#[derive(Debug, Parser)]
#[command(name = "status")]
#[command(version = "1.01")]
struct Args {
}

#[tokio::main]
async fn main() -> ErrStr<()> {
   let pivs = fetch_lines("main").await?;
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
