use std::{
   cmp::Ordering,
   collections::HashMap,
   fmt::Debug,
   fs::File,
   io::BufReader,
   path::Path
};

use chrono::{DateTime,NaiveDate};

extern crate serde;

use serde_json::{from_reader,from_str};

use book::{
   err_utils::{err_or,ErrStr}
};

use crate::types::{PivotDict,Price,Quote,RawPrices,Token,TokenId,Chart};

use reqwest::Client;

type Blob = String;

async fn gecko_fetcher(auth: &str, url: &str, params: &[(&str, &str)])
      -> ErrStr<Blob> {
   let client = Client::new();
   let req = client.get(url)
                   .query(params)
                   .header("accept", "application/json")
                   .header("x-cg-demo-api-key", auth);
   let res = err_or(req.send().await, "sending GET request to coingecko")?;
   let json = err_or(res.text().await, "parsing result body from coingecko")?;
   Ok(json)
}

async fn fetch_prices0(auth: &str, ids: &Vec<TokenId>) -> ErrStr<Blob> {
   let ids: &str = &ids.join(",");
   let params = [("ids", ids), ("vs_currencies", "usd")];
   let url = "https://api.coingecko.com/api/v3/simple/price";
   gecko_fetcher(auth, url, &params).await
}

pub async fn fetch_prices(auth: &str, dict: &PivotDict) -> ErrStr<RawPrices> {
   let ids: Vec<TokenId> = dict.left_values().map(String::to_string).collect();
   let raw = fetch_prices0(auth, &ids).await?;
   Ok(raw_to_prices(&raw))
}

fn raw_to_prices(raw: &Blob) -> RawPrices {
   from_str(raw).expect("JSON'd!")
}

// transforms JSON with token-ids to vec with token symbols

pub fn transform_prices(dict: &PivotDict, pric: &RawPrices) -> Vec<Price> {
   fn arr_m<'a>((k,v): (&'a TokenId, &'a Quote))
         -> impl Fn(&Token) -> Option<Price> + 'a {
      move |x| Some(((k.to_string(), x.to_string()), v.clone()))
      // or compose!(Some)(first(|k| (k.to_string(), x.to_string())))
   }

   let mut rows: Vec<Price> = pric.into_iter()
          .filter_map(|entry| dict.get_by_left(entry.0).and_then(arr_m(entry)))
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

// ----- Chart-data, or, fetching historical data for tokens -----------------

type StampedPrice0 = Vec<f64>;
type StampedData0<A> = Vec<A>;
type Chart0<A> = HashMap<String, StampedData0<A>>;

fn read_chart_from_file0<P: AsRef<Path> + Debug + Clone>(path: P)
        -> ErrStr<Chart0<StampedPrice0>> {
    // Open the file in read-only mode with buffer.
    let p = path.clone();
    let file = err_or(File::open(p), &format!("Cannot open {:?}", path))?;
    let reader = BufReader::new(file);      

    // Read the JSON contents of the file as an instance of the chart-data
    let chart = err_or(from_reader(reader), "Cannot parse JSON")?;

    Ok(chart)
}

pub fn read_chart_from_file<P: AsRef<Path> + Debug + Clone>(path: P)
        -> ErrStr<Chart<f64>> {      
   let raw = read_chart_from_file0(path)?;
   let mut ans = HashMap::new();
   fn to_stamp(v: &Vec<f64>) -> (NaiveDate, f64) {
      let dt = DateTime::from_timestamp((v[0] / 1000.0) as i64, 0).unwrap();
      (dt.date_naive(), v[1])
   }
   for (k,v) in raw {
      ans.insert(k, v.iter().map(to_stamp).collect());
   }
   Ok(ans)
}

/*
So, to scan a chart from REST endpoint, we do a from_str instead of from_reader

The curl command (use gecko_reader()):

curl --request GET \
     --url 'https://api.coingecko.com/api/v3/coins/bitcoin/market_chart?vs_currency=usd&days=42&interval=daily' \
     --header 'accept: application/json' \
     --header 'x-cg-demo-api-key: [API key]

n.b.: the URL, itself, embeds the token-id
*/

pub async fn fetch_chart0(auth: &str, tok_id: &str, days: i64) -> ErrStr<Blob> {
   let day6: &str = &format!("{days}");
   let ps = [("days", day6), ("vs_currency", "usd"), ("interval", "daily")];
   let url =
      format!("https://api.coingecko.com/api/v3/coins/{tok_id}/market_chart");
   gecko_fetcher(auth, &url, &ps).await
}

