use book::{
   err_utils::{err_or,ErrStr},
   rest_utils::read_rest,
   string_utils::to_string
};

use crypto::rest_utils::data_res;

pub fn read_pivots() -> ErrStr<Vec<String>> {
   let res = err_or(read_rest(&data_res("pivot-quiz-02", "pivots.csv")),
                    "Error reading pivots.csv from github")?;
   let lines = res.split("\n").map(to_string).collect();
   Ok(lines)
}
