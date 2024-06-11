// reads an order book from a REST endpoint using book::rest_utils

use book::{
   err_utils::{ErrStr,err_or},
   rest_utils::read_rest
};

type ErrStrStr = ErrStr<String>;

pub fn gecko_res(res: &str) -> String { 
   format!("https://api.kujira.app/api/coingecko/{res}")
}

fn git_lg_url() -> String {
   "https://raw.githubusercontent.com/logicalgraphs".to_string()
}

fn rez(dir: &str, branch: &str, res: &str) -> String {
   format!("{}/crypto-n-rust/{branch}/data-files/{dir}/{res}", git_lg_url())
}

pub fn fin_res(branch: &str, res: &str) -> String {
   rez("FIN", branch, res)
}

pub fn data_res(branch: &str, res: &str) -> String {
   rez("csv", branch, res)
}

/*
Remember to put the following on main()

#[tokio::main]

and

tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
*/

pub async fn read_orders_json(order_book: &str, depth: usize) -> ErrStrStr {
   let endpoint = format!("{}?ticker_id={order_book}&depth={depth}",
                          gecko_res("orderbook"));
   err_or(read_rest(&endpoint).await,
          &format!("Could not read orderbook {order_book}"))
}

pub async fn read_market_json() -> ErrStrStr {
   err_or(read_rest(&gecko_res("tickers")).await,
          "Could not read FIN marketplace.")
}

pub async fn read_aliases(aliases_url: &str) -> ErrStrStr {
   err_or(read_rest(aliases_url).await,
          &format!("Could not read aliases from {aliases_url}"))
}
