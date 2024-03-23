// despaces from stdin

use book::{
   compose,
   stream_utils::lines_from_stream
};


fn main() {
   let lines = lines_from_stream();
   fn print_it(s: String) { println!("{}", s); }
   lines.into_iter().for_each(compose!(print_it)(line1));
}

fn from1(s: &str) -> String {
   s.to_string() + " "
}

fn line1(s: String) -> String {
   s.split_whitespace().map(from1).collect()
}
