use std::collections::HashMap;

use bunsen::libs::{
   order_books::{OrderBook, parse_orderbook},
   purchases::buy,
   reports::{report_buy, report_roi}
};

use meth::{
   stride::fetch_stride_lsds,
   types::{LSD,exchange_rate,token}
};

use book::{
   csv_utils::print_csv,
   err_utils::ErrStr,
   utils::get_args
};

use crypto::rest_utils::read_orders_json;

fn usage() {
   let url = "https://api.kujira.app/api/coingecko/orderbook";
   let tick = "ticker_id=LOCAL_USK&depth=10";

   println!("\n./burn <ntoks> <book>");
   println!("\tParses <book> into an order book, then buys base with <ntoks>");
   let msg = "from <exchange rate> and <burn period>";
   println!("\tWe compute burn ROI and APR {msg}.\n");
   println!("\te.g.: {url}?{tick}\n");
}

fn main() -> ErrStr<()> {
   let args = get_args();
   let mut success = false;
   if let [amt, order_book] = args.as_slice() {
      let amount: f32 = amt.parse().expect(&format!("{amt} is not a number"));
      let file = read_orders_json(&order_book, 30)?;
      let book = parse_orderbook(&file)?;
      let lsds1 = fetch_stride_lsds()?;
      let lsds: HashMap<String, LSD> =
         lsds1.into_iter().map(|l| (token(&l), l)).collect();
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
