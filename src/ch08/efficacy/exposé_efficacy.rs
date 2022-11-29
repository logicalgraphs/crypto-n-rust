// We read in a path then compute the efficacy of trading on that path

// improvement: use only the paths and derive the ratios from marketplace
// sordid: we sort by trade results
// exposé: we exposé the interim trades

// ... also, I moved a lot of this stuff to the algos-library

use book::{
   list_utils::ht,
   utils::get_args
};

use crypto::{
   types::marketplace::read_marketplace,
   algos::paths::{process_paths,print_path}
};

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
                  for file in files {
                     println!("For file {}:", &file);
                     let paths = process_paths(ntoks, &marketplace)(&file);
                     paths.iter().for_each(print_path(ntoks));
                  }
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
