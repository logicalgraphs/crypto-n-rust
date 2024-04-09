use book::{
   file_utils::lines_from_file,
   utils::get_args
};

use crypto::parsers::token_prices::read_prices;

use cillaz::{
   parsers::{process_liquidations_by_date},
   reports::report
};

// ----- Main -------------------------------------------------------

fn usage() -> bool {
   println!("./cillaz <date> <prices CSV> <liquidations LSV>");
   println!("\nSlices and dices liquidations on ORCA (by day and by market)");
   true
}

fn main() {
   if let [date, prices, liquids] = get_args().as_slice() {
      let prces = read_prices(&prices);
      let lines = lines_from_file(&liquids);
      let jours = process_liquidations_by_date(&prces, &lines);
      report(&date, &jours);
   } else {
      usage();
   }
}
