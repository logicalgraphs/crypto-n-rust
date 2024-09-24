use std::fmt;

use book::{
   err_utils::ErrStr,
   utils::get_args
};

use swerve::{
   snarf::snarf_assets,
   types::build_trade_routes
};

fn usage() -> ErrStr<()> {
   println!("./answer22 <portfolio>
	Parses <portfolio>, extracting tokens and amounts then builds pivot
	trade-routes.");
   Err("Must include <portfolio> file!".to_string())
}

fn main() -> ErrStr<()> {
   let args = get_args();
   if let Some(file) = args.first() {
      let pools = snarf_assets(&file)?;
      for (blockchain, assets) in pools {
         println!("For blockchain {blockchain}:");
         for (_prime, asset) in assets {
            let trade_routes = build_trade_routes(&asset);
            fn vec_as_string<T: fmt::Display>(v: Vec<T>) -> String {
               v.iter().map(|e| format!("{e}")).collect::<Vec<_>>().join(", ")
            }
            println!(" * Trade-routes for assets {} are\n\t{}",
                     vec_as_string(asset.keys().collect()),
                     vec_as_string(trade_routes));
         }
      }
      Ok(())
   } else {
      usage()
   }
}
