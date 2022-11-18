// we read in the marketplace and write out a csv WHICH WE GRAPH!

use book::{
   file_utils::lines_from_file,
   list_utils::{tail,ht},
   utils::get_args
};

fn usage() {
   println!("./efficacy ntokens <graph paths>");
   println!("\n\tcomputes the number of tokens after trading a path.");
}

fn main() {
   if let (Some(toks), files) = ht(get_args()) {
      let ntoks: f32 = toks.parse().expect("number of tokens");
      files.iter().for_each(process_paths(ntoks));
      println!("\nCaput apres defero.");
   } else {
      usage();
   }
}

fn process_paths(ntoks: f32) -> impl Fn(&String) -> () {
   move |file: &String| {
      let lines = lines_from_file(file);
      tail(lines).iter().for_each(process_path(ntoks));
   }
}

fn process_path(ntoks: f32) -> impl Fn(&String) -> () {
   |line: &String| {
      let mut odd = true;
      let mut sum = 0.0;
      for datum in line.split(',') {
         if odd {
            let rat: f32 = datum.parse().exp
         }
         odd = !odd;
      }
   }
}
