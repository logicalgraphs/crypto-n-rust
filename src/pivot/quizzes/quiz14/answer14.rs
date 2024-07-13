use book::{
   csv_utils::{CsvWriter,print_csv},
   err_utils::{ErrStr,err_or},
   json_utils::AsJSON,
   list_utils::ht,
   utils::get_args
};

use swerve::{
   snarf::snarf_emas,
   types::{mk_rec,rec,mk_deltas,confidence}
};

fn usage() -> ErrStr<()> {
   println!("\n./rekt [--CSV] [--help|-h] <days> <token1> <token2>");
   println!("\tSnarfs pivots.csv and ratios <token1>/<token2> for <days>");
   println!("\tIt also computes the EMA20s for that token-pair,");
   println!("\tthen issues a buy- or sell-call.\n");
   println!("The --CSV-option directs rekt to output analysis as CSV.");
   println!("The -h or --help-option prints this message.");
   Err("Need to EMA20 over <days> <token1> <token2>".to_string())
}

#[tokio::main]
async fn main() -> ErrStr<()> {
   let args = get_args();
   let (fst, rest) = ht(&args);
   let (help, csv, args1) = if let Some(opt) = fst {
      if opt == "-h" || opt == "--help" {
         (true, false, &args)
      } else {
         if opt == "--CSV" {
            (false, true, &rest)
         } else {
            (false, false, &args)
         }
      }
   } else {
      (true, false, &args)
   };
   if help {
      usage()
   } else {
      doit(csv, args1).await
   }
}

async fn doit(csv: bool, args: &Vec<String>) -> ErrStr<()> {
   if let [dayz, token1, token2] = args.as_slice() {
      let days: u64 = err_or(dayz.parse(), &format!("{dayz} is not a number"))?;
      let emas = snarf_emas(days, token1, token2).await?;
      let deltas = mk_deltas(&emas);
      println!("\n{}\n", if csv { deltas.as_csv() } else { deltas.as_json() });
      let call = mk_rec(&emas);
      println!("{}\n", rec(&call));
      print_csv(&call);
      confidence(&deltas);
      Ok(())
   } else {
      usage()
   }
}
