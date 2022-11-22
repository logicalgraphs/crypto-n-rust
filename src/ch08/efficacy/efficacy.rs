// we read in the marketplace and write out a csv WHICH WE GRAPH!

use book::{
   file_utils::lines_from_file,
   list_utils::{tail,ht,parse_nums_opt},
   string_utils::str_str,
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
   move |line: &String| {
      let nums: Vec<String> = line.split(',').map(str_str).collect();
      println!("For {}:", line);
      let ans: f32 = parse_nums_opt(nums).iter().product::<f32>() * ntoks;
      println!("\t{} tokens becomes {}.", ntoks, ans);
   }
}
