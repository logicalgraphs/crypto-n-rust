use std::fmt;

use clap::Parser;

use book::err_utils::ErrStr;

use swerve::{
   snarf::{snarf_assets,snarf_quotes},
   types::{build_trade_routes, mk_trade_call, print_trade_call}
};

/// Makes trade-calls for <portfolio>
#[derive(Debug, Parser)]
#[command(name = "dawn")]
struct Args {
   /// portfolio of assets to pivot
   portfolio: String,

   /// minimum amount to pivot
   #[arg(short, long, default_value_t = 300.0)]
   min: f32
}

#[tokio::main] async fn main() -> ErrStr<()> {
   let args = Args::parse();
   let pools = snarf_assets(&args.portfolio)?;
   let (_, table, max_date) = snarf_quotes("main").await?;
   let min_swap = args.min;
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
            let mb = mk_trade_call(&table, &max_date, 100, &asset,
                                   &route, min_swap)?;
            if let Some((dt, call, conf)) = mb { 
               print_trade_call(&call, &dt, conf);
               swapped = true;
            }
         }
         if !swapped { println!("\t(no swaps)"); }
      }
   }
   Ok(())
}
