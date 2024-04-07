use book::{
   list_utils::ht,
   num_utils::parse_or,
   utils::{get_args,pred}
};

use crypto::types::books::parse_books_with_aliases;

use tob::reports::reportage;

fn usage() {
   println!("\n./top_order_books [--raw] <date> [minimum 24hr-volume=50000.0]");
   println!("\tGives the top order books by volume");
   println!("\t--raw flag gives exact volumes and percent-analyses.");
   println!("\tWhen specifying raw, please specify a minimum 24hr-volume,");
   println!("\tas there are many order books on FIN now.\n");
}

fn main() {
   let args = get_args();
   let mut success = false;
   if let (Some(frist), r1) = ht(&args) {
      let raw = frist == "--raw";
      if let Some(date) = if !raw { Some(frist) } else { r1.first().cloned() } {
         success = true;
         let (_, books) = parse_books_with_aliases(&date);
         let min_opt = pred(raw, parse_or(r1.last(), 50000.0));
         reportage(&date, &books, min_opt);
      }
   }
   if !success {
      usage();
   }
}
