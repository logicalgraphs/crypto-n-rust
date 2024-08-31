use std::env::var;

use book::{
   date_utils::parse_date,
   err_utils::{err_or,ErrStr},
   utils::get_args
};

use swerve::{
   fetch_prices::fetch_chart0,
   snarf::snarf_pivots
};

fn usage() {
   println!("./answer18 <date>
\tFetches, well, one of the charts for $PIVOTS of the last n days
\tn is computed from the last date recorded on $PIVOTS to <date>
");
}

#[tokio::main]
async fn main() -> ErrStr<()> {
   let pass = err_or(var("COIN_GECKO_API_KEY"),
                     "Could not fetch API key from environment")?;
   let args = get_args();
   if let Some(date) = args.first() {
      let today = parse_date(&date)?;
      let (dict, _pivots, max_date) = snarf_pivots().await?;
      if let Some((tok_id,tok)) = dict.iter().next() {
         let n = (today - max_date).num_days();
         let json = fetch_chart0(&pass, &tok_id, n).await?;
         println!("JSON for {tok} for last {n} days is:\n\n{json}");
      } else {
         panic!("Pivot table has no token ids!");
      }
   } else {
      usage();
   }
   Ok(())
}
