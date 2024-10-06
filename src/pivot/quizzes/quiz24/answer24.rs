use std::fmt;

use book::{
   err_utils::ErrStr,
   list_utils::ht,
   num_utils::parse_or,
   utils::get_args
};

use swerve::{
   snarf::{snarf_assets,snarf_pivots},
   types::{build_trade_routes, mk_trade_call, print_trade_call_d}
};

fn usage() -> ErrStr<()> {
   println!("
./dawn [--debug] <portfolio> [min_swap_ammount=500.00]
	Makes trade-calls for <portfolio>. The swap amount must be at least
        [min_swap_amount].

        When --debug is requested, ./dawn outputs state for each recommendation.
");
   Err("Must include <portfolio> file!".to_string())
}

#[tokio::main]
async fn main() -> ErrStr<()> {
   let args = get_args();
   if let (Some(flag), rest) = ht(&args) {
      cont(&flag == "--debug", flag, rest).await
   } else {
      usage()
   }
}

async fn cont(debug: bool, flag: String, args: Vec<String>) -> ErrStr<()> {
   if let Some(file) = if debug { args.first().cloned() } else { Some(flag) } {
      let pools = snarf_assets(&file)?;
      let (_, table, max_date) = snarf_pivots().await?;
      let min_swap = parse_or(args.last(), 500.0);
      println!("./dawn.\n\nRecommendations for {}; min_swap is ${}.",
               max_date, min_swap);
      for (blockchain, assets) in pools {
         println!("\nFor blockchain {blockchain}:");
         for (prime, asset) in assets {
            let trade_routes = build_trade_routes(&prime, &asset);
            fn vec_as_string<T: fmt::Display>(v: Vec<T>) -> String {
               v.iter().map(|e| format!("{e}")).collect::<Vec<_>>().join(", ")
            }
            println!("\nTrade-routes for assets {} are\n   {}",
                     vec_as_string(asset.keys().collect()),
                     vec_as_string(trade_routes.clone()));
            let mut swapped = false;
            for route in trade_routes {
               let mb_swap = mk_trade_call(&table, &max_date, 100, &asset,
                                           &route, min_swap)?;
               if let Some((dt, call, conf)) = mb_swap { 
                  print_trade_call_d(&call, &dt, conf, debug);
                  swapped = true;
               }
            }
            if !swapped { println!("\t(no swaps)"); }
         }
      }
      Ok(())
   } else {
      usage()
   }
}
