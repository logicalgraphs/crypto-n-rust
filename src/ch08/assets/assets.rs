use std::{
   collections::HashSet,
   path::Path
};

use book::{
   utils::get_args,
   file_utils::lines_from_file,
   csv_utils::print_csv,
   list_utils::{head,tail}
};

use crypto::types::{
   assets::{Asset,read_csv_asset,merge_assets,split_asset,mk_asset}
};

fn usage() {
   println!("\n./assets <assets CSV file>");
   println!("\n\tprints the assets in your portfolio.");
}

fn main() {
   if let Some(filename) = head(get_args()) {
      println!("Processing {}", filename);
      let mut assets = parse_n_print(filename);

      println!("\nOkay, now I'm going to sell 10 $ATOM for $13 ea");
      let sold = mk_asset("ATOM".to_string(), 10.0, 13.0);
      downdate(&mut assets, sold);
      print_assets(&assets);
   } else {
      usage();
   }
}

fn parse_n_print(file: impl AsRef<Path>) -> HashSet<Asset> {
   let lines = tail(lines_from_file(file));
   let mut assets = HashSet::new();
   lines.iter().for_each(|line| parse_n_add(line, &mut assets));
   print_assets(&assets);
   assets
}

fn parse_n_add(line: &String, assets: &mut HashSet<Asset>) {
   match read_csv_asset(line) {
      Ok(asset) => update(assets, asset),
      Err(msg) => println!("{}", msg)
   }
}

fn print_assets(assets: &HashSet<Asset>) {
   println!("asset,amount,quote");
   assets.iter().for_each(print_csv);
}

fn update(assets: &mut HashSet<Asset>, a: Asset) {
   assets.replace(match assets.get(&a) {
      Some(d) => merge_assets(d, a),
      None    => a
   });
}

fn downdate(assets: &mut HashSet<Asset>, a: Asset) {
   match assets.get(&a) {
      Some(c) => {
         match split_asset(c, a) {
            Some(d) => { assets.replace(d); },
            None    => { assets.remove(&c); }
         }
      }
      None    => { }
   }
}
