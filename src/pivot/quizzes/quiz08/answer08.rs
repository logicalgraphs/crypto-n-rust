use book::err_utils::ErrStr;

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
      Ok(())
   } else {
      Err("No last pivot!".to_string())
   }
}
