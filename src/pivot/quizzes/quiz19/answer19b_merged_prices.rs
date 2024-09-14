use book::{
   csv_utils::print_csv,
   date_utils::parse_date,
   err_utils::ErrStr,
   table_utils::merge,
   utils::{get_args,get_env}
};

use swerve::snarf::{snarf_pivots,snarf_pivot_table};

fn usage() {
   println!("./lizard <date>
\tFetches all charts for $PIVOTS of the last n days.
\tn is computed from the last date recorded on $PIVOTS to <date>.
");
}

// This answer snarfs the JSON then reifies that as a PivotTable... for all
// pivot-assets

#[tokio::main]
async fn main() -> ErrStr<()> {
   let pass = get_env("COIN_GECKO_API_KEY")?;
   let args = get_args();
   if let Some(date) = args.first() {
      let today = parse_date(&date)?;
      let (dict, _pivots, max_date) = snarf_pivots().await?;
      if dict.is_empty() { panic!("Pivot table has no token ids!"); }
      let n = (today - max_date).num_days();
      let mut pivots = None;
      for (tok_id, sym) in dict {
         println!("Snarfing {sym} chart...");
         let table = snarf_pivot_table(&pass, &tok_id, &sym, n).await?;
         pivots = pivots.and_then(|p| merge(&p, &table).ok()).or(Some(table));
      }
      pivots.and_then(|p| Some(print_csv(&p)));
   } else {
      usage();
   }
   Ok(())
}
