// name-explanation: when you've got to fetch-all, you're snarfin' it!
// ... am I right, fam? 😎

use std::{ops::Sub, time::Duration};
use async_std::task;

use chrono::{Days,NaiveDate};

use book::{
   csv_utils::parse_csv,
   date_utils::parse_date,
   err_utils::ErrStr,
   file_utils::extract_date_and_body,
   list_utils::tail,
   num_utils::parse_num_or_zero,
   table_utils::{col,ingest,row_filter,rows,from_map,transpose},
   types::tagged::{Tag,untag},
   utils::get_env
};

use crate::{
   fetch_quotes::{fetch_lines,parse_keys_symbols},
   fetch_prices::{Blob,fetch_prices,transform_prices,
                  fetch_chart_json,parse_chart},
   types::{Chart,Diffs,EMAs,mk_emas,Price,PivotDict,PivotTable,
           StampedData,Token,TokenId,mk_token,asset_parser,Pools,build_pools},
   verify::verify
};

// the el biggie en-snarf-ifier!

pub async fn snarf() -> ErrStr<(Vec<Price>, Option<Diffs>)> {
   let quotes = fetch_lines().await?;
   let dict = parse_keys_symbols(&quotes);
   let pass = get_env("COIN_GECKO_API_KEY")?;
   let raw_prices = fetch_prices(&pass, &dict).await?;
   let errs = verify(&dict, &raw_prices);
   let prices = transform_prices(&dict, &raw_prices);
   // or, with arrows: (verify &&& transform_prices) (&dict, &raw_prices)
   Ok((prices, errs))
}

// snarfs the Pivots-table and gives the most recent row-date

pub async fn snarf_quotes() -> ErrStr<(PivotDict, PivotTable, NaiveDate)> {
   let quotes = fetch_lines().await?;
   let dict = parse_keys_symbols(&quotes);
   fn token_or(s: &str) -> ErrStr<Token> { Ok(mk_token(s)) }
   let table: PivotTable =
      ingest(parse_date, token_or, parse_num_or_zero, &tail(&quotes), ",")?;
   let dates: Vec<NaiveDate> = rows(&table);
   let date: &NaiveDate = dates.last().ok_or("pivot table empty?")?;
   Ok((dict, table, date.clone()))
}

pub fn snarf_emas(table: &PivotTable, date: &NaiveDate, for_rows: u64,
                  t1: &Token, t2: &Token) -> ErrStr<EMAs> {
   let days = Days::new(for_rows);
   let start = date.sub(days);

   fn in_range(d: &NaiveDate) -> impl Fn(&NaiveDate) -> bool + '_ {
      |date| { date.ge(d) }
   }
   let domain = row_filter(in_range(&start), &table);
   let a = col(&domain, t1).expect(&format!("NO TOKEN NAMED {t1}"));
   let b = col(&domain, t2).expect(&format!("NO TOKEN NAMED {t2}"));

   let ratios: Vec<f32> =
      a.clone().into_iter()
               .zip(b.clone().into_iter())
               .map(|(a,b)| a / b)
               .collect();

   let dates = rows(&domain);
   let emas = mk_emas(t1, t2, 20, &dates, &ratios);
   Ok(emas)
}

// gets a symbol's historical price data, known as its 'chart'

async fn snarf_chart(auth: &str, tok_id: &TokenId, symbol: &Token,
                     days: i64) -> ErrStr<Tag<Chart<f32>>> {
   async fn chart_fetcher(auth: &str, tok_id: &TokenId, days: i64)
         -> ErrStr<Blob> {
      fetch_chart_json(auth, tok_id, days).await
   }
   let json = chart_fetcher(auth, tok_id, days).await?;
   let json1 = if json.trim() == "Throttled" {
      let problem = "request to coingecko REST endpoint throttled";
      let remedy = "pausing 60 seconds";
      println!("*** {problem}; {remedy} ...");
      task::sleep(Duration::from_secs(60)).await;
      chart_fetcher(auth, tok_id, days).await?
   } else {
      json
   };
   parse_chart(symbol, json1)
}

pub async fn snarf_pivot_table(auth: &str, tok_id: &TokenId, symbol: &Token,
                               days: i64) -> ErrStr<PivotTable> {
   let chart = snarf_chart(auth, tok_id, symbol, days).await?;
   let (_tag, value) = untag(&chart);
   let stamped_prices: &StampedData<f32> =
      value.get("prices").expect(&format!("price for {symbol}/{tok_id}"));
   let table = from_map(symbol, stamped_prices);
   Ok(transpose(&table))
}

pub fn snarf_assets(file: &str) -> ErrStr<Pools> {
   let (_date, lines) = extract_date_and_body(&file)?;
   let blocks = parse_csv(0, &asset_parser, &tail(&lines))?;
   let pools = build_pools(&blocks);
   Ok(pools)
}
