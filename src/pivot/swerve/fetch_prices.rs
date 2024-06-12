use book::err_utils::{err_or,ErrStr};

use reqwest::Client;

pub async fn fetch_prices(auth: &str, toks: &Vec<String>) -> ErrStr<String> {
   let client = Client::new();
   let ids: &str = &toks.join(",");
   let params = [("ids", ids), ("vs_currencies", "usd")];
   let url = "https://api.coingecko.com/api/v3/simple/price";
   let req = client.get(url)
                   .query(&params)
                   .header("accept", "application/json")
                   .header("x-cg-demo-api-key", auth);
   let res = err_or(req.send().await, "sending GET request to coingecko")?;
   let json = err_or(res.text().await, "parsing result body from coingecko")?;
   Ok(json)
}
