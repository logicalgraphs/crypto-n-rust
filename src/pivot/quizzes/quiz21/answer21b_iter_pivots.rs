use book::{
   csv_utils::print_csv,
   date_utils::parse_date,
   err_utils::ErrStr,
   utils::get_args
};

use swerve::{
   recs::rec,
   snarf::{snarf_assets,snarf_pivots},
   types::{mk_token,print_confidence}
};

fn usage() -> ErrStr<()> {
   println!("./answer21 <date> <portfolio>
	Parses <portfolio>, extracting tokens and amounts then makes trade-
	recommendations for <date> (which is today, btw).
");
   Err("Must include today's <date> and <portfolio> file!".to_string())
}

#[tokio::main]
async fn main() -> ErrStr<()> {
   let args = get_args();
   if let [date, file] = args.as_slice() {
      let today = parse_date(&date)?;
      let pools = snarf_assets(&file)?;
      println!("Pools are {pools:?}\n");

      // TODO: add build_curve_pools()

      let (_, table, max_dt) = snarf_pivots().await?;
      let btc = mk_token("BTC");
      let eth = mk_token("ETH");
      let (rekt, conf) = rec(&table, &max_dt, 100, &btc, &eth)?;
      print_csv(&rekt);
      print_confidence(&today, &conf);
      Ok(())
   } else {
      usage()
   }
}
