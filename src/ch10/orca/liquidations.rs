use book::{
   file_utils::lines_from_file,
   utils::get_args
};

use crypto::{
   parsers::token_prices::read_prices,
   types::aliases::load_aliases_graph
};

use cillaz::{
   parsers::{process_liquidations_by_date},
   reports::report
};

// ----- Main -------------------------------------------------------

fn usage() {
   println!("./cillaz <date> <prices CSV> <liquidations LSV>");
   println!("\nSlices and dices liquidations on ORCA (by day and by market)");
   println!("\nUses aliases.csv to resolve the au courant .axl tokens.");
}

fn main() {
   if let [date, prices, liquids] = get_args().as_slice() {
      let prces = read_prices(&prices);
      let lines = lines_from_file(&liquids);
      let aliases = load_aliases_graph();
      let jours = process_liquidations_by_date(&prces, &lines, &aliases);
      report(&date, &jours);
   } else {
      usage();
   }
}
