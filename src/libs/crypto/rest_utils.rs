// reads an order book from a REST endpoint using book::rest_utils

use book::{
   err_utils::{ErrStr,err_or},
   rest_utils::read_rest
};

type ErrStrStr = ErrStr<String>;

pub fn gecko_res(res: &str) -> String { 
   format!("https://api.kujira.app/api/coingecko/{res}")
}

fn raw_graphs() -> String {
   "https://raw.githubusercontent.com/logicalgraphs".to_string()
}

pub fn graphs_fin_res(res: &str) -> String {
   format!("{}/crypto-n-rust/main/data-files/FIN/{res}", raw_graphs())
}

pub fn read_orders_json(order_book: &str, depth: usize) -> ErrStrStr {
   let endpoint = format!("{}?ticker_id={order_book}&depth={depth}",
                          gecko_res("orderbook"));
   err_or(read_rest(&endpoint),
          &format!("Could not read orderbook {order_book}"))
}

pub fn read_market_json() -> ErrStrStr {
   err_or(read_rest(&gecko_res("tickers")), "Could not read FIN marketplace.")
}

pub fn read_aliases(aliases_url: &str) -> ErrStrStr {
   err_or(read_rest(aliases_url),
          &format!("Could not read aliases from {aliases_url}"))
}
