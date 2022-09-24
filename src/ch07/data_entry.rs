use book::{
   file_utils::lines_from_file,
   list_utils::{head,split},
   utils::get_args
};

fn usage() {
   println!("\n./data_entry <file>");
   println!("\n\tConvert file of quotes and amounts to just amounts.\n");
}

fn main() {
   if let Some(file) = head(get_args()) {
      if let [supplied, borrowed] =
           split(lines_from_file(file),"BORROWED".to_string()).as_slice() {
         preprocess_with_title("SUPPLIED", supplied.to_vec());
         preprocess_with_title("BORROWED", borrowed.to_vec());
      }
   } else {
      usage();
   }
}

fn preprocess_with_title(title: &str, mut lines: Vec<String>) {
   println!("\n{}\n", title);
   lines.retain(two);
   for line in lines {
      if let Some(position) = head(line.split(' ').collect()) {
         println!("{}", position);
      }
   }
}

fn two(line: &String) -> bool {
   let words: Vec<&str> = line.split(' ').collect();
   words.len() > 1 && head(words) != Some("Balance:")
}
