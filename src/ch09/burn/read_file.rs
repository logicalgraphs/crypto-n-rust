use book::{
   file_utils::lines_from_file,
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
      let file = lines_from_file(&filename).join(" ");
      reportage(&filename, &file);
   }
   if !success {
      usage();
   }
}

fn reportage(filename: &str, body: &str) {
   println!("From {filename} I got {body}");
}
