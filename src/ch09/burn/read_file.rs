use book::{
   file_utils::read_file,
   utils::get_args
};

fn usage() {
   println!("\n./burn book");
   println!("\tReads an order book from file.");
}

fn main() {
   let args = get_args();
   let mut success = false;
   if let Some(filename) = args.first() {
      success = true;
      let file = read_file(filename);
      reportage(&filename, &file);
   }
   if !success {
      usage();
   }
}

/*
 * moved to file_utils

pub fn read_file(filename: &str) -> String {
   lines_from_file(&filename).join(" ")
}

 */

fn reportage(filename: &str, body: &str) {
   println!("From {filename} I got {body}");
}
