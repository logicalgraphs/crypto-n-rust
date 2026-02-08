use book::err_utils::ErrStr;

use swerve::read_rest::read_quotes;

fn usage() {
   println!("./answer02");
   println!("\tReads data from a REST endpoint.\n");
}

#[tokio::main]
async fn main() -> ErrStr<()> {
   usage();
   let quotes = read_quotes().await?;
   println!("The first five lines of quotes.csv on github:\n");
   for line in quotes.into_iter().take(5) {
      println!("{line}");
   }
   Ok(())
}
