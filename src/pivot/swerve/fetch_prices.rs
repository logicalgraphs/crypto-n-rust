use std::cmp::Ordering;

extern crate serde;

use serde_json::from_str;

use book::{
   err_utils::{err_or,ErrStr}
};

use crate::types::{Dict,Price,Quote,RawPrices,Token,TokenId};

use reqwest::Client;

type Blob = String;

async fn fetch_prices0(auth: &str, ids: &Vec<TokenId>) -> ErrStr<Blob> {
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

fn raw_to_prices(raw: &Blob) -> RawPrices {
   from_str(raw).expect("JSON'd!")
}

pub async fn fetch_prices(auth: &str, dict: &Dict) -> ErrStr<RawPrices> {
   let ids: Vec<TokenId> = dict.keys().map(String::to_string).collect();
   let raw = fetch_prices0(auth, &ids).await?;
   Ok(raw_to_prices(&raw))
}

pub fn transform_prices(dict: &Dict, pric: &RawPrices) -> Vec<Price> {
   fn arr_m<'a>((k,v): (&'a TokenId, &'a Quote))
         -> impl Fn(&Token) -> Option<Price> + 'a {
      move |x| Some(((k.to_string(), x.to_string()), v.clone()))
   }

   let mut rows: Vec<Price> = pric.into_iter()
          .filter_map(|entry| dict.get(entry.0).and_then(arr_m(entry)))
                 // much easier with monads and arrows, seriously! :<
          .collect();
   fn root(s: &str) -> String {
      s.trim_matches(char::is_lowercase).to_string()
   }
   fn cmp(a: &str, b: &str) -> Ordering {
      root(a).cmp(&root(b)).then(a.len().cmp(&b.len()))
   }
   rows.sort_by(|((_,a), _), ((_,b), _)| cmp(a, b));
   rows
}
