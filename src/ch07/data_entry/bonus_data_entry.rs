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
      let dater = lines_from_file(file);
      let (_hdr, body) = dater.split_at(2);
      split(body.to_vec(),"BORROWED".to_string()).iter_mut()
          .fold(1.0, preprocess_with_sign);
   } else {
      usage();
   }
}

// a function that 'folds over' the (negative) sign
fn preprocess_with_sign(sign: f32, lines: &mut Vec<String>) -> f32 {
   lines.retain(two);
   for line in lines {
      if let Some(position) = head(line.split(' ').collect()) {
         let mut p1 = position.to_string();
         p1.retain(no(','));
         let num: f32 = p1.parse().expect("Not a number");
         println!("{}", sign * num);
      }
   }
   sign * -1.0
}

fn two(line: &String) -> bool {
   let words: Vec<&str> = line.split(' ').collect();
   words.len() > 1 && head(words) != Some("Balance:")
}

// a function that returns a function
fn no(ch: char) -> impl Fn(char) -> bool {
   move |chr| chr != ch
}
