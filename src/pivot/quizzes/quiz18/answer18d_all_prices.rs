use book::{
   csv_utils::print_csv,
   date_utils::parse_date,
   err_utils::ErrStr,
   utils::{get_args,get_env}
};

use swerve::snarf::{snarf_quotes,snarf_pivot_table};

fn usage() {
   println!("./answer18 <date>
\tFetches all charts for $QUOTES of the last n days.
\tn is computed from the last date recorded on $QUOTES to <date>.
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
      let (dict, _pivots, max_date) = snarf_quotes().await?;
      if dict.is_empty() { panic!("Pivot table has no token ids!"); }
      for (tok_id, sym) in dict {
         let n = (today - max_date).num_days();
         let table = snarf_pivot_table(&pass, &tok_id, &sym, n).await?;
         println!("JSON for {sym} for last {n} days is:\n");
         print_csv(&table);
      }
   } else {
      usage();
   }
   Ok(())
}
