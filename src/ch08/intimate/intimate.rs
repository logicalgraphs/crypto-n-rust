// We read in a path then compute the efficacy of trading on that path

// This is for ./emily, wherever I may find her, ...

// 🎵 Time it was! Oh, what a time it was. It was ...
//    A time of innocence.
//    A time of confidences.

//    Long ago, it must be, ...
//    I had a photograph.
//    Perserved her memories.
//    They're all that's left me. 🎶

use book::{
   list_utils::ht,
   string_utils::str_string,
   utils::get_args
};

use crypto::{
   types::marketplace::read_marketplace,
   algos::paths::{process_with_path,print_path}
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
                     let le_path: Vec<String> =
                        path.split(',').map(str_string).collect();
                     match process_with_path(ntoks, &marketplace, &le_path) {
                        Some(thing) => print_path(ntoks)(&thing),
                        None => println!("Sure'n b'gorra, but I canna do this.")
                     }
                  }
                  println!("\nSlán go fóill, love.")
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
