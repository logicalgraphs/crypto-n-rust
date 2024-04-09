// instead of buying tokens, we sell them.

use bunsen::entries::{OrderBook, parse_orderbook, sell, report_sale};

use book::{
   csv_utils::print_csv,
   utils::get_args
};

use crypto::rest_utils::read_orders;

fn usage() {
   println!("\n./sell <ntoks> <book>");
   println!("\tParses <book> into an order book, then sells base with <ntoks>");
}

fn main() -> Result<(), String> {
   let args = get_args();
   let mut success = false;
   if let [amt, order_book] = args.as_slice() {
      let file = read_orders(&order_book, 30)?;
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
