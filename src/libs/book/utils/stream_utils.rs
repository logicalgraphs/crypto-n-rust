use std::io::stdin;

pub fn lines_from_stream() -> Vec<String> {
   let mut lines: Vec<String> = Vec::new();
   loop {
      let mut butter = String::new();
      stdin().read_line(&mut butter).expect("EOF");
      if butter == "" { break; }
      lines.push(butter.trim().to_string());
   }
   lines
}
