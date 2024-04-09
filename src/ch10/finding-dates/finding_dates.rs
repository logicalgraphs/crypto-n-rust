use crypto::parsers::find_date::find_date;

fn main() {
   test_line("SOL     2024-01-08 07:36:39");
   test_line("2024-01-08 00:36:45");
}

fn test_line(line: &str) {
   println!("Testing line: {line}...");
   let msg = match find_date(line) {
      Ok(date) => format!("found date: {date}"),
      Err(str) => format!("!!! Could not parse line; Error: {str}")
   };
   println!("{msg}");
}
