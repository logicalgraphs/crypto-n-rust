// ./vern'

// Given current market conditions and sets of isomorphic paths,
// ./vern' finds the efficacious paths A -> B where B is the isomorphism
// and A is a node (or vertex) in paths B.

// History:

// We read in a path then compute the efficacy of trading on that path

// improvement: use only the paths and derive the ratios from marketplace
// sordid: we sort by trade results
// exposé: we exposé the interim trades

// ... also, I moved a lot of this stuff to the algos-library

use book::{
   string_utils::str_string,
   utils::get_args
};

use crypto::{
   types::marketplace::read_marketplace,
   algos::paths::{paths_processor,process_with_path,print_path}
};

fn usage() {
   let m = "<marketplace file>";
   let g = "<graph paths CSV file>";
   println!("./vern ntokens start-token end-token {m} {g}");
   println!("\n\tcomputes the number of tokens after trading a path.\n");
}

fn main() {
   let args = get_args();
   if args.len() < 4 { usage(); } else { go(&args); }
}

fn go(args: &Vec<String>) {
   let mut cont = false;
   let (args1, files) = args.split_at(4);
   if let [toks, stok, etok, marketplace] = args1 {
      cont = !files.is_empty();
      if cont {
         println!("./vern, my main man, ./vern!\n");
         match toks.parse() {
            Ok(ntoks) => {
               let market = read_marketplace(marketplace);
               let pathf = |line: &String| {
                  let raw_path: Vec<&str> = line.split(',').collect();
                  fn str_str_str(s: &&str) -> String {
                     str_string(*s)
                  }
                  let lst: &str = etok;
                  if raw_path.last() == Some(&lst) {
                     let path: Vec<String> =
                        raw_path.iter()
                                .skip_while(|n| n != &&stok)
                                .map(str_str_str)
                                .collect();
                     process_with_path(ntoks, &market, &path) 
                  } else {
                     None
                  }
               };
               for file in files {
                  println!("For file {}:", &file);
                  let paths = paths_processor(&pathf, &file);
                  paths.iter().for_each(print_path(ntoks));
               }
               println!("\nHey, Ray! 😊");
            },
            Err(_) => { cont = false; }
         }
      }
   }

   if !cont {
      usage();
   }
}
