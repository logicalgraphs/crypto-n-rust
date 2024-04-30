use book::{
   file_utils::lines_from_file,
   utils::get_args
};

use crypto::{
   parsers::token_prices::read_prices,
   types::aliases::load_aliases_from_file
};

use cillaz::{
   parsers::{process_liquidations_by_date},
   reports::report
};

// ----- Main -------------------------------------------------------

fn usage() -> bool {
   println!("./cillaz <date> <prices CSV> <liquidations LSV> <aliases CSV>");
   println!("\nSlices and dices liquidations on ORCA (by day and by market)");
   true
}

fn main() {
   if let [date, prices, liquids, aliasus] = get_args().as_slice() {
      let prces = read_prices(&prices);
      let lines = lines_from_file(&liquids);
      let aliases = load_aliases_from_file(&aliasus);
      let jours = process_liquidations_by_date(&prces, &lines, &aliases);
      report(&date, &jours);
   } else {
      usage();
   }
}
