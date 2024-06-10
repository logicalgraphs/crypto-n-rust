use book::err_utils::ErrStr;

use swerve::read_rest::read_pivots;

fn usage() {
   println!("./quiz02");
   println!("\tReads data from a REST endpoint, ... LIBRARYITIZED!\n");
}

fn main() -> ErrStr<()> {
   usage();
   let res = read_pivots()?;
   println!("First five lines of the pivots database:\n");
   for line in res.into_iter().take(5) {
      println!("{line}");
   }
   Ok(())
}
