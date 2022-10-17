use book::{
   utils::get_args,
   file_utils::lines_from_file,
   csv_utils::print_csv,
   list_utils::head
};

use crypto::types::Asset;

fn usage() {
   println!("\n./assets <assets CSV file>");
   println!("\n\tprints the assets in your portfolio.");
}

fn main() {
   if let Some(filename) = head(get_args()) {
      println!("Processing {}", filename);
   } else {
      usage();
   }
}
