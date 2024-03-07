use std::io::stdin;

use book::{
   html_utils::{AsHTML,mk_table},
   string_utils::to_string
};

fn main() {
   let mut lines: Vec<Vec<String>> = Vec::new();
   loop {
      let mut butter = String::new();
      stdin().read_line(&mut butter).expect("EOF");
      if butter == "" { break; }
      lines.push(butter.trim().split("\t").map(to_string).collect());
   }
   println!("{}", mk_table(&lines).as_html());
}

// sample table at fin.tsv
// sample result at fin.html
