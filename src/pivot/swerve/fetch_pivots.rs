use book::{
   err_utils::ErrStr,
   rest_utils::read_rest,
   string_utils::to_string
};

use crypto::rest_utils::data_res;

use crate::types::Pivots;

pub async fn fetch_pivots() -> ErrStr<Pivots> {
   let url = data_res("pivot-quiz-06-answer", "pivots.csv");
   let res = read_rest(&url).await?;
   let lines: Vec<String> = res.split("\n").map(to_string).collect();
   Ok(lines)
}
