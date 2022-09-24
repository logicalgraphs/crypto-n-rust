use book::{
   file_utils::lines_from_file,
   utils::{get_args,head}
};

fn usage() {
   println!("\n./data_entry <file>");
   println!("\n\tConvert file of quotes and amounts to just amounts.\n");
}

fn main() {
   if let Some(file) = head(get_args()) {
      let (supplied, _borrowed) = lines_from_file(file).split_at("BORROWED");
      preprocess(supplied);
   } else {
      usage();
   }
}

fn preprocess(mut lines: Vec<String>) -> Vec<String> {
   lines.retain(|line| two(line));
   for line in &lines {
      println!("{}", line);
   }
   lines
}

fn two(line: &String) -> bool {
   let words: Vec<&str> = line.split(' ').collect();
   words.len() > 1 && head(words) != Some("Balance:")
}
