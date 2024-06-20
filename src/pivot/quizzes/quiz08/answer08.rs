use chrono::{NaiveDate}; // ,format::parse_and_remainder};

use book::{
   err_utils::{err_or,ErrStr},
   list_utils::{parse_nums,tail},
   string_utils::to_string
};

use swerve::snarf::snarf_pivots;

fn usage() {
   println!("\n./stat");
   println!("\tSnarfs pivots.csv and reports the latest numbers.");
}

#[tokio::main]
async fn main() -> ErrStr<()> {
   usage();
   let (pivs, _dict) = snarf_pivots().await?;
   if let Some(line) = pivs.last() {
      println!("My last pivot is {line}");
      let (date, nums) = parse_row(&line)?;
      println!("Date: {date}");
      println!("pivots: {nums:?}");
      Ok(())
   } else {
      Err("No last pivot!".to_string())
   }
}

fn parse_row(row: &str) -> ErrStr<(NaiveDate, Vec<f32>)> {
   let (date, line) = err_or(NaiveDate::parse_and_remainder(row, "%Y-%m-%d"),
                             &format!("Unable to parse date from '{row}'"))?;
   let cols: Vec<String> = line.split(",").map(to_string).collect();
   let nums = parse_nums(tail(&cols));
   Ok((date, nums))
}
