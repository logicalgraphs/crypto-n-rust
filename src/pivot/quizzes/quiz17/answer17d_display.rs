use std::fmt::Debug;

use chrono::NaiveDate;

use book::err_utils::ErrStr;

use swerve::{
   fetch_prices::read_chart_from_file,
   types::StampedData
};

fn main() -> ErrStr<()> {
    let chart = read_chart_from_file("data/eth.json")?;
    for section in chart {
       print_section(&section);
    }
   Ok(())
}

fn print_section<A: Debug + Clone>((section, row): &(String, StampedData<A>)) {
   println!("Section: {section}");

   fn print_datum<A: Debug>(data: &A) {
      println!("\t{:?}", data);
   }
   let mut prices: Vec<(NaiveDate, A)> = Vec::new();
   // ugh: row.into_iter().cloned().collect();
   for (k,v) in row { prices.push((k.clone(), v.clone())); }
   prices.sort_by_key(|k| k.0);
   prices.iter().take(3).for_each(print_datum);
   println!("\t...");
}
