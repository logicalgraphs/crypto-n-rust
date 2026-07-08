use chrono::NaiveDate;
use clap::Parser;

use book::{
   csv_utils::print_csv,
   err_utils::ErrStr,
   table_utils::merge,
   utils::get_env
};

use swerve::snarf::{snarf_quotes,snarf_quote_table};

/// Fetches all charts for $QUOTES of the last n days.
///
/// lizard snarfs the JSON then reifies that as a PivotTable... for all
/// pivot-assets
#[derive(Debug, Parser)]
#[command(name = "lizard")]
struct Args {
   /// today's date to reconstitute quotes to
   date: NaiveDate
}

#[tokio::main] async fn main() -> ErrStr<()> {
   let pass = get_env("COIN_GECKO_API_KEY")?;
   let args = Args::parse();
   let (dict, _pivots, max_date) = snarf_quotes("main").await?;
   if dict.is_empty() { panic!("Pivot table has no token ids!"); }
   let n = (args.date - max_date).num_days();
   let mut pivots = None;
   for (tok_id, sym) in dict {
      println!("Snarfing {sym} chart...");
      let table = snarf_quote_table(&pass, &tok_id, &sym, n).await?;
      pivots = pivots.and_then(|p| merge(&p, &table).ok()).or(Some(table));
   }
   pivots.and_then(|p| Some(print_csv(&p)));
   Ok(())
}
