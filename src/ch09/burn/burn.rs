use bunsen::{
   entries::{OrderBook, parse_orderbook, buy, report_buy, report_roi},
   read_rest::read_orders
};

use meth::{
   stride::fetch_stride_lsds,
   types::{exchange_rate,token}
};

use book::{
   csv_utils::print_csv,
   list_utils::assoc_list,
   utils::get_args
};

fn usage() {
   let url = "https://api.kujira.app/api/coingecko/orderbook";
   let tick = "ticker_id=LOCAL_USK&depth=10";

   println!("\n./burn <ntoks> <book>");
   println!("\tParses <book> into an order book, then buys base with <ntoks>");
   let msg = "from <exchange rate> and <burn period>";
   println!("\tWe compute burn ROI and APR {msg}.\n");
   println!("\te.g.: {url}?{tick}\n");
}

fn main() -> Result<(), String> {
   let args = get_args();
   let mut success = false;
   if let [amt, order_book] = args.as_slice() {
      let amount: f32 = amt.parse().expect(&format!("{amt} is not a number"));
      let file = read_orders(&order_book, 30)?;
      let book = parse_orderbook(&file)?;
      let lsds1 = fetch_stride_lsds()?;
      let lsds = assoc_list(lsds1, token);
      if let Some(lsd) = lsds.get(&book.base) {
         let rate = exchange_rate(lsd);
         let burn = lsd.unbond;
         success = true;
         reportage(amount, rate, burn as f32, &book);
      } else {
         panic!("Cannot find LSD for {}", book.base)
      }
   }
   if !success {
      usage();
   }
   Ok(())
}

fn reportage(amount: f32, rate: f32, burn: f32, book: &OrderBook) {
   print_csv(book);

   let purchase = buy(book, amount);
   println!("\n{}", report_buy(&book, amount, &purchase));

   println!("\n{}, exchange rate: {rate}, unbonding period: {burn}", book.base);
   println!("{}", report_roi(rate, burn, &purchase));
}
