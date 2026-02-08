use book::{
   err_utils::ErrStr,
   json_utils::AsJSON,
   num_utils::parse_num,
   utils::get_args
};

use swerve::{snarf::snarf_emas,types::mk_token};

fn usage() -> ErrStr<()> {
   println!("\n./ema <days> <token1> <token2>");
   println!("\tSnarfs quotes.csv and ratios <token1>/<token2> for <days>");
   println!("\tIt also computes the EMA20s for that token-pair.");
   Err("Need to EMA20 over <days> <token1> <token2>".to_string())
}

#[tokio::main]
async fn main() -> ErrStr<()> {
   let args = get_args();
   if let [dayz, token1, token2] = args.as_slice() {
      let days = parse_num(&dayz)?;
      let t1 = mk_token(&token1);
      let t2 = mk_token(&token2);
      let emas = snarf_emas(days as u64, &t1, &t2).await?;
      println!("emas = {};", emas.as_json());
      Ok(())
   } else {
      usage()
   }
}
