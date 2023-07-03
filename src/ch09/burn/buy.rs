use bunsen::entries::{OrderBook, parse_orderbook, buy, report_purchase};

use book::{
   csv_utils::print_csv,
   file_utils::read_file,
   utils::get_args
};

fn usage() {
   println!("\n./burn <ntoks> <book>");
   println!("\tParses <book> into an order book, then buys base with <ntoks>");
}

fn main() -> Result<(), String> {
   let args = get_args();
   let mut success = false;
   if let Some(filename) = args.first() {
      let file = read_file(&filename);
      let book = parse_orderbook(&file)?;
      success = true;
      reportage(&book);
   }
   if !success {
      usage();
   }
   Ok(())
}

fn reportage(book: &OrderBook) {
   print_csv(book);

   let purchase = buy(book, 10.0);
   println!("\n{}", report_purchase("USK", 10.0, &purchase));
}
