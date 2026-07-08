use clap::Parser;

use book::{
   csv_utils::{CsvWriter,print_csv},
   err_utils::ErrStr,
   json_utils::AsJSON
};

use swerve::{
   snarf::{snarf_emas,snarf_quotes},
   types::{mk_rec,rec_as_string,mk_deltas,confidence,mk_token}
};

/// Snarfs quotes.csv and ratios <token1>/<token2> for <days>
///
/// It also computes the EMA20s for that token-pair,
/// then issues a buy- or sell-call.
#[derive(Debug, Parser)]
#[command(name = "rekt")]
#[command(version = "1.01")]
struct Args {
   /// number of days to compute trend-lines, e.g. 100
   days: u64,

   /// primary asset of pivot pool, e.g.: BTC
   primary: String,

   /// pivot asset of pivot pool, e.g.: ETH
   pivot: String,

   /// output as CSV?
   #[arg(short, long)]
   csv: bool
}

#[tokio::main]
async fn main() -> ErrStr<()> {
   let args = Args::parse();
   rekt_it(args.csv, args.days, &args.primary, &args.pivot).await
}

async fn rekt_it(csv: bool, days: u64, token1: &str, token2: &str)
      -> ErrStr<()> {
    let t1 = mk_token(&token1);
    let t2 = mk_token(&token2);
    let (_headers, quotes, date) = snarf_quotes("main").await?;
    let emas = snarf_emas(&quotes, &date, days, &t1, &t2)?;
    let deltas = mk_deltas(&emas);
    println!("\n{}\n", if csv { deltas.as_csv() } else { deltas.as_json() });
    let call = mk_rec(&emas);
    println!("{}\n", rec_as_string(&call));
    print_csv(&call);
    confidence(&deltas);
    Ok(())
}
