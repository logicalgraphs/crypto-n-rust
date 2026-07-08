use clap::Parser;

use book::{ err_utils::ErrStr, json_utils::AsJSON };

use swerve::{snarf::{snarf_quotes,snarf_emas},types::mk_token};

/// Snarfs quotes.csv and ratios <token1>/<token2> for <days>
///
/// It also computes the EMA20s for that token-pair.
#[derive(Debug, Parser)]
#[command(name = "ema")]
struct Args {
   /// The duration to compute the EMA
   days: u64,

   /// Denominator token, e.g.: BTC
   denom: String,

   /// Numerator token, e.g.: ETH
   numer: String
}

#[tokio::main]
async fn main() -> ErrStr<()> {
   let args = Args::parse();
   let t1 = mk_token(&args.denom);
   let t2 = mk_token(&args.numer);
   let (_, quotes, date) = snarf_quotes("main").await?;
   let emas = snarf_emas(&quotes, &date, args.days, &t1, &t2)?;
   println!("emas = {};", emas.as_json());
   Ok(())
}
