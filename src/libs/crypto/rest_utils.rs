// reads an order book from a REST endpoint using book::rest_utils

use book::rest_utils::read_rest;

pub fn read_orders(order_book: &str, depth: i8)
    -> Result<String, String> {
   let url = "https://api.kujira.app/api/coingecko/orderbook";
   let endpoint = format!("{url}?ticker_id={order_book}&depth={depth}");
   match read_rest(&endpoint) {
      Ok(ans) => Ok(ans),
      Err(str) => Err(format!("Could not read orderbook {}. Error: {:?}",
                              order_book, str))
   }
}

pub fn read_markets() -> Result<String, String> {
   match read_rest("https://api.kujira.app/api/coingecko/tickers") {
      Ok(ans) => Ok(ans),
      Err(str) => Err(format!("Could not read FIN marketplace. Error: {str:?}"))
   }
}
