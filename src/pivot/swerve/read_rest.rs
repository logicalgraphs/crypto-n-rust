use book::{
   err_utils::{err_or,ErrStr},
   string_utils::to_string
};

use crypto::rest_utils::data_res;

pub async fn read_pivots() -> ErrStr<Vec<String>> {
   let url = data_res("pivot-quiz-02", "pivots.csv");
   let res = err_or(reqwest::get(&url).await,
                    "Error making get-reqwest call to github")?;
   let body = err_or(res.text().await,
                    "Error reading pivots.csv on github")?;
   let lines: Vec<String> = body.split("\n").map(to_string).collect();
   Ok(lines)
}
