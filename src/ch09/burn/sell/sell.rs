// instead of buying tokens, we sell them.

use bunsen::{
   entries::{OrderBook, parse_orderbook},
   purchases::sell,
   reports::report_sale
};

use book::{
   csv_utils::print_csv,
   file_utils::read_file,
   utils::get_args
};

fn usage() {
   let url = "https://api.kujira.app/api/coingecko/orderbook";
   let tick = "ticker_id=LOCAL_USK&depth=10";
   println!("\n./sell <ntoks> <book>");
   println!("\tParses <book> into an order book, then sells <ntoks>");
   println!("\n\te.g.: {url}?{tick}\n");
}

fn main() -> Result<(), String> {
   let args = get_args();
   let mut success = false;
   if let [amt, filename] = args.as_slice() {
      let file = read_file(&filename);
      let book = parse_orderbook(&file)?;
      let amount: f32 = amt.parse().expect(&format!("{amt} is not a number"));
      success = true;
      reportage(amount, &book);
   }
   if !success {
      usage();
   }
   Ok(())
}

fn reportage(amount: f32, book: &OrderBook) {
   print_csv(book);

   let purchase = sell(book, amount);
   println!("\n{}", report_sale(book, amount, &purchase));
}
