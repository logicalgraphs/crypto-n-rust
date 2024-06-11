use std::{
   collections::HashMap,
   iter::zip
};

use book::{
   err_utils::ErrStr,
   string_utils::to_string,
   utils::get_args
};

use swerve::read_rest::read_pivots;

fn usage() {
   println!("./answer03 <token coingecko API id>");
   println!("\tgives the token-symbol for the provided <id>");
}

#[tokio::main]
async fn main() -> ErrStr<()> {
   let args = get_args();
   let mb_id = args.first();
   if let Some(id) = mb_id {
      let res = read_pivots().await?;
      let ids = res[0].split(",").skip(1).map(to_string);
      let syms = res[1].split(",").skip(1).map(to_string);
      let dict: HashMap<String, String> = zip(ids, syms).collect();
      let sym = dict.get(id).unwrap();
      println!("The sym for {id} is {sym}");
   } else {
      usage();
   }
   Ok(())
}
