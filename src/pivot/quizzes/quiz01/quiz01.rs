use book::err_utils::ErrStr;

fn usage() {
   println!("./quiz01");
   println!("\tReads data from a REST endpoint.");
}

fn main() -> ErrStr<()> {
   usage();

   // do something here to read the REST endpoint
   // URL: https://github.com/logicalgraphs/crypto-n-rust/blob/pivot-quiz-01/data-files/csv/quotes.csv
   Ok(())
}
