use book::{
   utils::get_args,
   list_utils::head
};

use crypto::types::marketplace::{read_marketplace,fetch_orderbooks};

fn usage() {
   println!("\n./marketplace <marketplace LSV file>");
   println!("\n\tprints the marketplace tokens and prices");
}

fn main() {
   if let Some(filename) = head(get_args()) {
      println!("Processing {}", filename);
      let markets = read_marketplace(filename);
      let atom_books = fetch_orderbooks(&markets, &"ATOM".to_string());
      println!("The ATOM order books are:");
      for o in atom_books.iter() {
         println!("\t{}", o);
      }
   } else {
      usage();
   }
}
