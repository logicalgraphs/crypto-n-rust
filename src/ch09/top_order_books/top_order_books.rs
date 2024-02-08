use book::{
   list_utils::{head,ht},
   utils::get_args
};

use crypto::rest_utils::read_markets;

use tob::reports::reportage;

fn usage() {
   println!("\n./top_order_books [--raw] <date> [minimum 24hr-volume=50000.0]");
   println!("\tGives the top order books by volume");
   println!("\t--raw flag gives exact volumes and percent-analyses.");
   println!("\tWhen specifying raw, please specify a minimum 24hr-volume,");
   println!("\tas there are many order books on FIN now.\n");
}

fn main() -> Result<(), String> {
   let args = get_args();
   let mut success = false;
   if let (Some(frist), r1) = ht(&args) {
      let raw = frist == "--raw";
      if let Some(date) = if !raw { Some(frist) } else { head(&r1) } {
         success = true;
         let json = read_markets()?;
         reportage(&date, &json, get_minimum(raw, r1.last(), 50000.0));
      }
   }
   if !success {
      usage();
   }
   Ok(())
}

fn get_minimum(raw: bool, min: Option<&String>, default: f32) -> Option<f32> {
   let mut ans = None;
   if raw {
      ans = Some(default);
      if let Some(str) = min {
         if let Ok(num) = str.parse() {
            ans = Some(num);
         }
      }
   }
   ans
}
