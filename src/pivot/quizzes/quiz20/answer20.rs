use book::{
   csv_utils::print_csv,
   date_utils::parse_date,
   err_utils::ErrStr,
   table_utils::rows,
   utils::{get_args,get_env}
};

use swerve::{
   snarf::{snarf_pivots,snarf_pivot_table},
   types::{mk_token}
};

fn usage() {
   println!("./tok <date> <API-id> <symbol>
\tFetches chart for $PIVOTS for entire date-range of $PIVOTS.
\tYou can find <API-id> from https://www.coingecko.com/
\t<symbol> is the token, e.g.: BTC or ETH or whatevs.
\t<date> is today.
");
}

// This answer snarfs the JSON then reifies that as a PivotTable... for a
// requested token-id

#[tokio::main]
async fn main() -> ErrStr<()> {
   let pass = get_env("COIN_GECKO_API_KEY")?;
   let args = get_args();
   if let [date, tok_id, sym] = args.as_slice() {
      let today = parse_date(&date)?;
      let (_dict, pivots, _max_date) = snarf_pivots().await?;
      let rows = rows(&pivots);
      let min_date = rows.first().ok_or("PIVOTS table empty???")?;
      let n = (today - *min_date).num_days() + 1;
      let token = mk_token(&sym);
      let table = snarf_pivot_table(&pass, &tok_id, &token, n).await?;
      print_csv(&table);
   } else {
      usage();
   }
   Ok(())
}
