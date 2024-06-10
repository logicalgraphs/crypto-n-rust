use book::err_utils::ErrStr;

use swerve::read_rest::read_pivots;

fn usage() {
   println!("./answer02");
   println!("\tReads data from a REST endpoint.\n");
}

fn main() -> ErrStr<()> {
   usage();
   let pivots = read_pivots()?;
   println!("The first five lines of pivots.csv on github:\n");
   for line in pivots.into_iter().take(5) {
      println!("{line}");
   }
   Ok(())
}
