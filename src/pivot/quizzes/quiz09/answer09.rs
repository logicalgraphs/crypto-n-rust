use book::{
   date_utils::parse_date,
   err_utils::ErrStr,
   utils::get_args
};

use swerve::{
   reports::{one_row,report_diffs},
   snarf::snarf
};

fn usage() -> ErrStr<()> {
   println!("\n./gecko <date> [branch=main]");
   println!("\tQueries coingecko REST endpoint for token-prices");
   Err("Enter date of data to query coingecko REST endpoint.".to_string())
}

#[tokio::main]
async fn main() -> ErrStr<()> {
   let args = get_args();
   if let Some(dt) = args.first() {
      let branch = if args.len() == 2 { &args.last().unwrap() } else { "main" };
      let dat = parse_date(&dt)?;
      let (prices, errs) = snarf(branch).await?;
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
