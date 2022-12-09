// convert tabs and spaces to commas from stdin

use std::io::stdin;

fn main() {
   loop {
      let mut butter = String::new();
      stdin().read_line(&mut butter).expect("Done.");
      if butter == "" { break; }
      let b2: String = butter.split(|c: char| c == '\t')
                             .map(append_comma)
                             .collect();
      println!("{}", b2.trim());
   }
}

fn append_comma(s: &str) -> String {
   s.to_string() + ","
}
