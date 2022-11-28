// We read in a path then compute the efficacy of trading on that path

// improvement: use only the paths and derive the ratios from marketplace

use std::collections::HashSet;

use book::{
   file_utils::lines_from_file,
   list_utils::{tail,ht,head},
   string_utils::str_string,
   utils::get_args
};

use crypto::types::marketplace::{OrderBook, read_marketplace, ratio_for};

fn usage() {
   println!("./efficacy ntokens <marketplace file> <graph paths CSV file>");
   println!("\n\tcomputes the number of tokens after trading a path.\n");
}

fn main() {
   let mut cont = false;
   if let (Some(toks), files1) = ht(get_args()) {
      if let (Some(market), files) = ht(files1) {
         cont = !files.is_empty();
         if cont {
            match toks.parse() {
               Ok(ntoks) => {
                  let marketplace = read_marketplace(market);
                  files.iter().for_each(process_paths(ntoks, &marketplace));
                  println!("\nCaput apres defero.");
               },
               Err(_) => { cont = false; }
            }
         }
      }
   }

   if !cont {
      usage();
   }
}

fn process_paths(ntoks: f32, market: &HashSet<OrderBook>)
   -> impl Fn(&String) -> () + '_ {
   move |file: &String| {
      let lines = lines_from_file(file);
      let mut paths: Vec<(f32, String)> = tail(lines).iter()
          .map(process_path(ntoks, market))
          .collect();

      paths.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

      paths.iter().for_each(print_path(ntoks));
   }
}

fn process_path(ntoks: f32, market: &HashSet<OrderBook>)
   -> impl Fn(&String) -> (f32, String) + '_ {
   move |line: &String| {
      let path: Vec<String> = line.split(',').map(str_string).collect();
      let ans: f32 = process_books(ntoks, market, &path);
      (ans, line.clone())
   }
}

fn print_path(ntoks: f32) -> impl Fn(&(f32, String)) -> () {
   move | p: &(f32, String) |  {
      let (ans, line) = p;
      println!("For {}:", line);
      println!("\t{} tokens becomes {}.", ntoks, ans);
   }
}

// this function is a comonadic extension, and so demonstrates `experiment`

fn process_books(ntoks: f32, market: &HashSet<OrderBook>, path: &Vec<String>)
   -> f32 {
   let mut ans = ntoks;
   if let (Some(from), tos) = ht(path.to_vec()) {
      if let Some(to) = head(tos.clone()) {
         ans=process_books(ntoks * ratio_for(market, &from, &to), market, &tos);
      }
   }
   ans
}
