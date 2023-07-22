use std::collections::HashMap;

use book::{
   csv_utils::parse_csv,
   utils::get_args
};

use meth::{
   read_rest::read_rest,
   types::{parse_lsds_without_burn, print_lsds, merge_burn_rates_d}
};

fn usage() {
   println!("./lsd <date>");
   println!("\tReads data from a REST endpoint.");
}

fn main() -> Result<(), String> {
   if let Some(date) = get_args().first() {
      let stroll = "Stride-Labs/stride/stakeibc/host_zone";
      let url = "https://stride-api.polkachu.com";
      let body = read_rest(&format!("{url}/{stroll}"))?;
      let burns = fetch_burns()?;
      let burnless_lsds = parse_lsds_without_burn(&body);
      let lsds = merge_burn_rates_d(&burnless_lsds, &burns, true);
      print_lsds(&date, &lsds);
   } else {
      usage();
   }
   Ok(())
}

fn fetch_burns() -> Result<HashMap<String,u8>, String> {
   let lg_url = "https://raw.githubusercontent.com/logicalgraphs";
   let burn_dir = "crypto-n-rust/assemblage/src/ch09/lsd/data/burn-rates.csv";
   let csv = read_rest(&format!("{lg_url}/{burn_dir}"))?;
   fn burn_f(row: &Vec<&str>) -> Result<Option<(String, u8)>, String> {
      if let [name, _, c, _] = row.as_slice() {
         let count: u8 = c.parse().expect(&format!("{c} is not a number"));
         Ok(Some((name.to_string(), count)))
      } else {
         Err(format!("{row:?} is not CSV-parseable!"))
      }
   }
   let mut lines = csv.lines();
   let rows = parse_csv(1, burn_f, &mut lines)?;
   let burns: HashMap<String, u8> = rows.into_iter().collect();
   Ok(burns)
}
