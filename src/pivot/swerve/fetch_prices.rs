use std::{
   cmp::Ordering,
   collections::HashMap
};

extern crate serde;

use serde_json::from_str;

use book::{
   err_utils::{err_or,ErrStr}
};

use crate::types::{Dict,Price,Quote,Token,TokenId};

use reqwest::Client;

type RawPrices = String;

async fn fetch_prices0(auth: &str, ids: &Vec<TokenId>) -> ErrStr<RawPrices> {
   let client = Client::new();
   let ids: &str = &ids.join(",");
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

fn raw_to_prices(raw: &RawPrices) -> HashMap<TokenId,Quote> {
   from_str(raw).expect("JSON'd!")
}

pub async fn fetch_prices(auth: &str, dict: &Dict) -> ErrStr<Vec<Price>> {
   let ids: Vec<TokenId> = dict.keys().map(String::to_string).collect();
   let raw = fetch_prices0(auth, &ids).await?;
   let pric = raw_to_prices(&raw);

   fn arr_m((k,v): (TokenId, Quote)) -> impl Fn(&Token) -> Option<Price> {
      move |x| Some(((k.to_string(), x.to_string()), v.clone()))
   }

   let mut rows: Vec<Price> = pric.into_iter()
          .filter_map(|entry| dict.get(&entry.0).and_then(arr_m(entry)))
                 // much easier with monads and arrows, seriously! :<
          .collect();
   fn root(s: &str) -> String {
      s.trim_matches(char::is_lowercase).to_string()
   }
   fn cmp(a: &str, b: &str) -> Ordering {
      root(a).cmp(&root(b)).then(a.len().cmp(&b.len()))
   }
   rows.sort_by(|((_,a), _), ((_,b), _)| cmp(a, b));
   Ok(rows)
}
