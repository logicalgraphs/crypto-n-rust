use book::{
   csv_utils::print_csv,
   err_utils::{ErrStr,err_or},
   utils::get_args
};

use swerve::{
   snarf::snarf_emas,
   types::{mk_rec,rec}
};

fn usage() -> ErrStr<()> {
   println!("\n./rekt <days> <token1> <token2>");
   println!("\tSnarfs pivots.csv and ratios <token1>/<token2> for <days>");
   println!("\tIt also computes the EMA20s for that token-pair,");
   println!("\tthen issues a buy- or sell-call.");
   Err("Need to EMA20 over <days> <token1> <token2>".to_string())
}

#[tokio::main]
async fn main() -> ErrStr<()> {
   let args = get_args();
   if let [dayz, token1, token2] = args.as_slice() {
      let days: u64 = err_or(dayz.parse(), &format!("{dayz} is not a number"))?;
      let emas = snarf_emas(days, token1, token2).await?;
      let call = mk_rec(&emas);
      println!("{}\n", rec(&call));
      print_csv(&call);
      Ok(())
   } else {
      usage()
   }
}
