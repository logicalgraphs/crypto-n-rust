use book::{
   csv_utils::print_csv,
   date_utils::parse_date,
   err_utils::ErrStr,
   utils::{get_args,get_env}
};

use swerve::snarf::{snarf_pivots,snarf_pivot_table};

fn usage() {
   println!("./answer18 <date>
\tFetches, well, one of the charts for $PIVOTS of the last n days
\tn is computed from the last date recorded on $PIVOTS to <date>
");
}

// This answer snarfs the JSON then reifies that as a PivotTable

#[tokio::main]
async fn main() -> ErrStr<()> {
   let pass = get_env("COIN_GECKO_API_KEY")?;
   let args = get_args();
   if let Some(date) = args.first() {
      let today = parse_date(&date)?;
      let (dict, _pivots, max_date) = snarf_pivots().await?;
      if let Some((tok_id, sym)) = dict.iter().next() { 
            // iter().next() is a complicated way of saying: first().
            // although, tbf, 'first' of a bijection is a weird request with
            // a perplexing job for implementors to provide a consistent answer.
         let n = (today - max_date).num_days();
         let table = snarf_pivot_table(&pass, &tok_id, &sym, n).await?;
         println!("JSON for {sym} for last {n} days is:\n");
         print_csv(&table);
      } else {
         panic!("Pivot table has no token ids!");
      }
   } else {
      usage();
   }
   Ok(())
}
