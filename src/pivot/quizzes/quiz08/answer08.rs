use std::iter::zip;

use chrono::NaiveDate;

use book::{
   err_utils::{err_or,ErrStr},
   list_utils::{parse_nums,tail},
   string_utils::to_string,
   utils::get_args
};

use swerve::{
   fetch_pivots::{fetch_lines,parse_token_headers},
   reports::portfolio_prices,
   types::{Price,mk_quote,Quote,TokenId,Token}
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

fn parse_row(row: &str) -> ErrStr<(NaiveDate, Vec<Quote>)> {
   let (date, line) = err_or(NaiveDate::parse_and_remainder(row, "%Y-%m-%d"),
                             &format!("Unable to parse date from '{row}'"))?;
   let cols: Vec<String> = line.split(",").map(to_string).collect();
   let nums = parse_nums(tail(&cols)).into_iter().map(mk_quote).collect();
   Ok((date, nums))
}

fn to_portfolio(hdr: Vec<(TokenId,Token)>, prices: Vec<Quote>) -> Vec<Price> {
   zip(hdr.into_iter(), prices.into_iter()).collect()
}
