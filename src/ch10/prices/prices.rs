
use book::utils::get_args;
use crypto::types::books::{load_books,prices};

fn usage() {
   println!("./prices <marketplace JSON>\n");
   println!("Outputs assets and their price-quotes.");
}

fn main() {
   if let [market] = get_args().as_slice() {
      println!("asset,quote");
      for (asset,price) in prices(&load_books(&market)) {
         println!("{asset},{price}");
      }
   } else { usage(); }
}
