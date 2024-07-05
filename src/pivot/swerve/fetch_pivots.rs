use std::iter::{Iterator,zip};

use chrono::NaiveDate;

use book::{
   date_utils::parse_date_and,
   err_utils::ErrStr,
   list_utils::{parse_nums,tail},
   rest_utils::read_rest,
   string_utils::to_string,
   utils::pred
};

use crypto::rest_utils::data_res;

use crate::types::{Dict,Pivots,mk_quote,Quote,TokenId,Token};

pub fn parse_keys_symbols(pivots: &Pivots) -> Dict {
   parse_token_headers(pivots).into_iter().collect()
}

pub fn parse_token_headers(pivots: &Pivots) -> Vec<(TokenId, Token)> {
   fn splitter(line: &str) -> impl Iterator<Item=String> + '_ {
      line.split(",").skip(1).map(to_string)
   }
   let ids = splitter(&pivots[0]);
   let syms = splitter(&pivots[1]);
   zip(ids, syms).collect()
}

pub async fn fetch_lines() -> ErrStr<Pivots> {
   let url = data_res("main", "pivots.csv");
   let res = read_rest(&url).await?;
   let lines: Pivots =
      res.lines().filter_map(|l| pred(!l.is_empty(), to_string(l))).collect();
   Ok(lines)
}

pub fn parse_row(row: &str) -> ErrStr<(NaiveDate, Vec<Quote>)> {
   let (date, line) = parse_date_and(row)?;
   let cols: Vec<String> = line.split(",").map(to_string).collect();
   let nums = parse_nums(tail(&cols)).into_iter().map(mk_quote).collect();
   Ok((date, nums))
}
