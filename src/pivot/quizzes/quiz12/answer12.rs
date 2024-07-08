use book::{
   err_utils::ErrStr,
   json_utils::AsJSON,
   num_utils::parse_num,
   utils::get_args
};

use swerve::snarf::snarf_emas;

fn usage() -> ErrStr<()> {
   println!("\n./ema <days> <token1> <token2>");
   println!("\tSnarfs pivots.csv and ratios <token1>/<token2> for <days>");
   println!("\tIt also computes the EMA20s for that token-pair.");
   Err("Need to EMA20 over <days> <token1> <token2>".to_string())
}

#[tokio::main]
async fn main() -> ErrStr<()> {
   let args = get_args();
   if let [dayz, token1, token2] = args.as_slice() {
      let days = parse_num(&dayz)?;
      let emas = snarf_emas(days as u64, &token1, &token2).await?;
      println!("emas = {};", emas.as_json());
      Ok(())
   } else {
      usage()
   }
}
