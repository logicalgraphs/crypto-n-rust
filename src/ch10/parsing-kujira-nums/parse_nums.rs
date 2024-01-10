use parse_nums::parse_kujira_num::parse_kujira_number;

use book::{
   file_utils::lines_from_file,
   list_utils::ht,
   utils::get_args
};

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
      let rest = if let Some(num) = parse_kujira_number(&lines) {
         println!("Processed {num}");
         let (_num, rest) = lines.split_at(2);
         rest.to_vec()
      } else {
         if let (Some(line), rest) = ht(&lines) {
            println!("\tDid not process line: {line}");
            rest
         } else { panic!("no more lines?!?") }
      };
      process_lines(&rest);
   }
}
