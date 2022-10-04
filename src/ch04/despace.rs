// despaces from stdin

use std::io::stdin;


fn main() {
   loop {
      let mut butter = String::new();
      stdin().read_line(&mut butter).expect("Done.");
      if butter == "" { break; }
      let b2: String = butter.split_whitespace().map(from1).collect();
      println!("{}", b2.trim());
   }
}

fn from1(s: &str) -> String {
   s.to_string() + " "
}
