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

use book::utils::get_args;

use crypto::{
   types::marketplace::read_marketplace,
   algos::paths::{process_paths_for,print_path}
};

fn usage() {
   let m = "<marketplace file>";
   let g = "<graph paths CSV file>";
   println!("./vern ntokens start-token {m} {g}");
   println!("\n\tcomputes the number of tokens after trading a path.\n");
}

fn main() {
   let args = get_args();
   if args.len() < 4 { usage(); } else { go(&args); }
}

fn go(args: &Vec<String>) {
   let mut cont = false;
   let (args1, files) = args.split_at(3);
   if let [toks, tok, marketplace] = args1.to_vec().as_slice() {
      cont = !files.is_empty();
      if cont {
         match toks.parse() {
            Ok(ntoks) => {
               let market = read_marketplace(marketplace);
               for file in files {
                  println!("For file {}:", &file);
                  let paths = process_paths_for(ntoks, tok, &market)(&file);
                  paths.iter().for_each(print_path(ntoks));
               }
            println!("\nCaput apres defero.");
         },
         Err(_) => { cont = false; }
      }
      }
   }

   if !cont {
      usage();
   }
}
