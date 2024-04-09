// one-line output from stdin

use book::stream_utils::lines_from_stream;

fn main() {
   let lines = lines_from_stream();
   let ls: Vec<String> = lines.into_iter().map(line1).collect();
   println!("{}", ls.join(""));
}

fn from1(s: &str) -> String {
   s.to_string() + " "
}

fn line1(s: String) -> String {
   s.split_whitespace().map(from1).collect()
}
