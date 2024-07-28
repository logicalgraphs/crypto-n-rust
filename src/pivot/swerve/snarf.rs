// name-explanation: when you've got to fetch-all, you're snarfin' it!
// ... am I right, fam? 😎

use std::ops::Sub;

use chrono::{Days,NaiveDate};

use book::{
   date_utils::parse_date,
   err_utils::ErrStr,
   list_utils::tail,
   num_utils::parse_num_or_zero,
   table_utils::{Table,col,ingest,row_filter,rows},
   utils::get_env
};

use crate::{
   fetch_pivots::{fetch_lines,parse_keys_symbols},
   fetch_prices::{fetch_prices,transform_prices},
   types::{Diffs,EMAs,mk_emas,Price},
   verify::verify
};

// the el biggie en-snarf-ifier!

pub async fn snarf() -> ErrStr<(Vec<Price>, Option<Diffs>)> {
   let pivs = fetch_lines().await?;
   let dict = parse_keys_symbols(&pivs);
   let pass = get_env("COIN_GECKO_API_KEY")?;
   let raw_prices = fetch_prices(&pass, &dict).await?;
   let errs = verify(&dict, &raw_prices);
   let prices = transform_prices(&dict, &raw_prices);
   // or, with arrows: (verify &&& transform_prices) (&dict, &raw_prices)
   Ok((prices, errs))
}

pub type Pivots = Table<NaiveDate, String, f32>;

// snarfs the Pivots-table and gives the most recent row-date

pub async fn snarf_pivots() -> ErrStr<(Pivots, NaiveDate)> {
   let pivs = fetch_lines().await?;
   fn to_string_or(s: &str) -> ErrStr<String> { Ok(s.to_string()) }
   let table: Pivots =
      ingest(parse_date, to_string_or, parse_num_or_zero, &tail(&pivs), ",")?;
   let dates: Vec<NaiveDate> = rows(&table);
   let date: &NaiveDate = dates.last().ok_or("pivot table empty?")?;
   Ok((table, date.clone()))
}

// this is a bit more than a snarf: I snarf the pivots then compute the
// EMAs for a pair. Fortunately the EMA-type self-computes, so it's an
// easy hand-off.

pub async fn snarf_emas(for_rows: u64, t1: &String, t2: &String)
      -> ErrStr<EMAs> {
   let days = Days::new(for_rows);
   let (table, date) = snarf_pivots().await?;
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
