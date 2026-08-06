use chrono::NaiveDate;
use clap::Parser;

use book::{
   parse_args_add_banner,
   cli_utils::generate_banner,
   err_utils::ErrStr
};

use swerve::{ reports::{ one_row, report_diffs }, snarf::snarf };

/// Queries coingecko REST endpoint for token-prices
#[derive(Debug, Parser)]
#[command(name = "gecko")]
#[command(version = "1.03")]
struct Args {
   /// date to query quotes, e.g.: $LE_DATE
   date: NaiveDate,

   /// repository branch to run quotes
   #[arg(short, long, default_value = "main")]
   branch: String
}

#[tokio::main]
async fn main() -> ErrStr<()> {
   let args = parse_args_add_banner!(Args);
   let (prices, errs) = snarf(&args.branch).await?;
   if let Some(diffs) = errs { 
      Err(report_diffs(&diffs))
   } else {
      one_row(&args.date, &prices)
   }
}
