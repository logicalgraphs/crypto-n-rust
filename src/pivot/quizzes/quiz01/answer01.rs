use book::{
   err_utils::ErrStr,
   rest_utils::read_rest
};

use crypto::rest_utils::data_res;

fn usage() {
   println!("./answer01");
   println!("\tReads data from a REST endpoint.\n");
}

#[tokio::main]
async fn main() -> ErrStr<()> {
   usage();
   let res = read_rest(&data_res("pivot-quiz-01-answer", "pivots.csv")).await?;
   println!("First five lines of the pivots database:\n");
   for line in res.split("\n").take(5) {
      println!("{line}");
   }
   Ok(())
}
