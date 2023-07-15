use bunsen::entries::{
   OrderBook, parse_orderbook, buy, report_purchase, report_roi
};

use book::{
   csv_utils::print_csv,
   file_utils::read_file,
   utils::get_args
};

fn usage() {
   let url = "https://api.kujira.app/api/coingecko/orderbook";
   let tick = "ticker_id=LOCAL_USK&depth=10";

   println!("\n./burn <ntoks> <exchange rate> <burn period> <book>");
   println!("\tParses <book> into an order book, then buys base with <ntoks>");
   let msg = "from <exchange rate> and <burn period>";
   println!("\tWe compute burn ROI and APR {msg}.\n");
   println!("\te.g.: {url}?{tick}\n");
}

fn main() -> Result<(), String> {
   let args = get_args();
   let mut success = false;
   if let [amt, ex_rate, period, filename] = args.as_slice() {
      let amount: f32 = amt.parse().expect(&format!("{amt} is not a number"));
      let rate: f32 =
         ex_rate.parse().expect(&format!("{ex_rate} is not a rate"));
      let burn: f32 =
         period.parse().expect(&format!("{period} is not a number of days."));
      let file = read_file(&filename);
      let book = parse_orderbook(&file)?;
      success = true;
      reportage(amount, rate, burn, &book);
   }
   if !success {
      usage();
   }
   Ok(())
}

fn reportage(amount: f32, rate: f32, burn: f32, book: &OrderBook) {
   print_csv(book);

   let purchase = buy(book, amount);
   println!("\n{}", report_purchase(&book.target, amount, &purchase));
   println!("\n{}", report_roi(rate, burn, &purchase));
}
