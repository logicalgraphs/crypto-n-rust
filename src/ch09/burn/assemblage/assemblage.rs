use std::collections::HashSet;

use mark::files::{files_d, collect_files_d};

// use book::util::get_args;

fn main() -> Result<(), String> {
   let files = collect_files_d("../data", true)?;
   doit("USK,wBTC,axlUSDC,wBTC,USK", &files);
   doit("wBTC,axlUSDC,USK,MNTA,wBTC", &files); // fails: I don't track $MNTA ... yet
   Ok(())
}

fn doit(trade: &str, files: &HashSet<String>) {
   println!("For trade {trade}");
   let toks = trade.split(",").collect();
   println!("{:?}", files_d(&toks, files, true));
}
