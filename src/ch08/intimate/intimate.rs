// We read in a path then compute the efficacy of trading on that path

use book::{
   list_utils::ht,
   utils::get_args
};

use crypto::{
   types::marketplace::read_marketplace,
   algos::paths::{process_path,print_path}
};

fn usage() {
   let csv = "<comma-separated path>";
   println!("./intimate ntokens <marketplace file> {csv}");
   println!("\n\tcomputes the number of tokens after trading a path.\n");
}

fn main() {
   let mut cont = false;
   if let (Some(toks), market_and) = ht(get_args()) {
      if let (Some(market), paths) = ht(market_and) {
         cont = !paths.is_empty();
         if cont {
            match toks.parse() {
               Ok(ntoks) => {
                  let marketplace = read_marketplace(market);
                  for path in paths { // there should be one path?
                     let thunk = process_path(ntoks, &marketplace)(&path);
                     print_path(ntoks)(&thunk);
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
