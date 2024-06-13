use book::{
   err_utils::ErrStr,
   utils::get_args
};

use swerve::{
   reports::report,
   snarf::snarf
};

fn usage() {
   println!("\n./answer06 <date>");
   println!("\tQueries coingecko REST endpoint for token-prices");
}

#[tokio::main]
async fn main() -> ErrStr<()> {
   let args = get_args();
   if let Some(date) = args.first() {
      let prices = snarf().await?;
      report(&date, &prices);
   } else {
      usage();
   }
   Ok(())
}
