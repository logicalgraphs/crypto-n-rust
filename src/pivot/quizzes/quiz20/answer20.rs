use chrono::NaiveDate;
use clap::Parser;

use book::{
   csv_utils::print_csv,
   err_utils::ErrStr,
   table_utils::rows,
   utils::get_env
};

use swerve::{
   snarf::{snarf_quotes,snarf_quote_table},
   types::{mk_token}
};

// This answer snarfs the JSON then reifies that as a PivotTable... for a
// requested token-id

/// Fetches chart for $QUOTES for entire date-range of $QUOTES.
///
/// You can find <API-id> from https://www.coingecko.com/
#[derive(Debug, Parser)]
#[command(name = "tok")]
#[command(version = "1.01")]
struct Args {
   /// token-symbol to fetch quotes, e.g.: BTC
   symbol: String,

   /// CoinGecko API token id, e.g.: bitcoin
   token_id: String,

   /// Date to which the prices are fetched (starting from $QUOTES-date),
   /// e.g.: $LE_DATE
   date: NaiveDate
}

#[tokio::main]
async fn main() -> ErrStr<()> {
   let pass = get_env("COIN_GECKO_API_KEY")?;
   let args = Args::parse();
   let (_dict, pivots, _max_date) = snarf_quotes("main").await?;
   let rows = rows(&pivots);
   let min_date = rows.first().ok_or("QUOTES table empty???")?;
   let n = (args.date - *min_date).num_days() + 1;
   let token = mk_token(&args.symbol);
   let table = snarf_quote_table(&pass, &args.token_id, &token, n).await?;
   print_csv(&table);
   Ok(())
}
