use book::{
   file_utils::lines_from_file,
   utils::get_args
};

use cillaz::{
   parsers::{process_liquidations_by_date,read_prices},
   reports::report
};

// ----- Main -------------------------------------------------------

fn usage() -> bool {
   println!("./cillaz <date> <prices CSV> <liquidations LSV>");
   println!("\nSlices and dices liquidations on ORCA (by day and by market)");
   true
}

fn main() {
   let mut okay = false;
   if let [date, prices, liquids] = get_args().as_slice() {
      let prces = read_prices(&prices);
      let lines = lines_from_file(&liquids);
      let jours = process_liquidations_by_date(&prces, &lines);
      report(&date, &jours);
      okay = true;
   }

   // #[allow(unused_must_use)]
   !okay && usage();
}
