use std::iter::{Iterator,zip};

use book::{
   compose,
   err_utils::ErrStr,
   rest_utils::read_rest,
   string_utils::to_string
};

use crypto::rest_utils::data_res;

use crate::types::{Dict,Pivots};

pub fn parse_keys_symbols(pivots: &Pivots) -> Dict {

   fn splitter(line: &str) -> impl Iterator<Item=String> + '_ {
      line.split(",").skip(1).map(compose!(to_string)(str::trim_end))
   }
   let ids = splitter(&pivots[0]);
   let syms = splitter(&pivots[1]);
   zip(ids, syms).collect()
}

pub async fn fetch_lines() -> ErrStr<Pivots> {
   let url = data_res("pivot", "pivots.csv");
   let res = read_rest(&url).await?;
   let lines: Pivots = res.split("\n").map(to_string).collect();
   Ok(lines)
}
