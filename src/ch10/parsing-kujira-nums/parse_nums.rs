use book::{
   file_utils::lines_from_file,
   list_utils::ht,
   utils::get_args
};

use crypto::parsers::kujira_nums::parse_kujira_number;

fn usage() {
   println!("\n./parse_nums <file with kujira nums>");
}

fn main() {
   let mut err = true;
   if let Some(filename) = get_args().first() {
      let lines = lines_from_file(&filename);
      process_lines(&lines);
      err = false;
   }

   if err { usage(); }
}

fn process_lines(lines: &Vec<String>) {
   if !lines.is_empty() {
      let rest = match parse_kujira_number(&lines) {
         Ok(num) => {
            println!("Processed {num}");
            let (_num, rest) = lines.split_at(2);
            rest.to_vec()
         },
         Err(str) => {
            if let (Some(line), rest) = ht(&lines) {
               println!("\tDid not process line: {line}\n\tReason: {str}");
               rest
            } else {
               panic!("no more lines?!?")
            }
         }
      };
      process_lines(&rest);
   }
}
