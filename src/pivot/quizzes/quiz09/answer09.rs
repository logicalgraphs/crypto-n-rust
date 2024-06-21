use book::{
   date_utils::date,
   err_utils::ErrStr,
   utils::get_args
};

use swerve::{
   reports::{one_row,report_diffs},
   snarf::snarf
};

fn usage() -> ErrStr<()> {
   println!("\n./gecko <date>");
   println!("\tQueries coingecko REST endpoint for token-prices");
   Err("Enter date of data to query coingecko REST endpoint.".to_string())
}

#[tokio::main]
async fn main() -> ErrStr<()> {
   let args = get_args();
   if let Some(dt) = args.first() {
      let dat = date(&dt)?;
      let (prices, errs) = snarf().await?;
      if let Some(diffs) = errs { 
         Err(report_diffs(&diffs))
      } else {
         one_row(&dat, &prices);
         Ok(())
      }
   } else {
      usage()
   }
}
