// reads an order book from a REST endpoint using book::rest_utils

use book::{
   err_utils::{ErrStr,err_or},
   rest_utils::read_rest
};

type ErrStrStr = ErrStr<String>;

pub fn read_orders_json(order_book: &str, depth: i8) -> ErrStrStr {
   let url = "https://api.kujira.app/api/coingecko/orderbook";
   let endpoint = format!("{url}?ticker_id={order_book}&depth={depth}");
   err_or(read_rest(&endpoint),
          &format!("Could not read orderbook {order_book}"))
}

pub fn read_market_json() -> ErrStrStr {
   err_or(read_rest("https://api.kujira.app/api/coingecko/tickers"),
         "Could not read FIN marketplace.")
}
