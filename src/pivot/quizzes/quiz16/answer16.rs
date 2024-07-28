use book::{
   date_utils::parse_date,
   err_utils::ErrStr,
   utils::get_args
};

use swerve::snarf::snarf_pivots;

fn usage() {
   println!("
./answer16 <date>
	Finds last update date for $PIVOTS and computes how many rows need 
	to be fetched.
");
}

#[tokio::main]
async fn main() -> ErrStr<()> {
   let args = get_args();
   if let Some(date) = args.first() {
      let (_table, max_date) = snarf_pivots().await?;
      let today = parse_date(&date)?;
      let n = (today - max_date).num_days();
      println!("The most recent update to $PIVOTS is {max_date}
I need to collect {n} days'-worth of pivots.");
   } else {
      usage();
   }
   Ok(())
}
