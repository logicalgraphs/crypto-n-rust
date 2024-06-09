use book::err_utils::ErrStr;

use swerve::read_pivots;

fn usage() {
   println!("./quiz01");
   println!("\tReads data from a REST endpoint.\n");
}

fn main() -> ErrStr<()> {
   usage();
   let lg = "https://raw.githubusercontent.com/logicalgraphs";
   let piv_data = "crypto-n-rust/pivot-quiz-01/data-files/csv/pivots.csv";
   let res = err_or(read_rest(&format!("{lg}/{piv_data}")),
                    "Error reading REST endpoint")?;
   println!("First five lines of the pivots database:\n");
   for line in res.split("\n").take(5) {
      println!("{line}");
   }
   Ok(())
}
