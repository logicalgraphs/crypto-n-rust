// gets the axlUSDC prices of all assets, including the ones that aren't
// paired with axlUSDC. How do we do this? Magic. PFM.

use std::{
   collections::HashMap,
   fmt::Display
};

use book::{
   utils::get_args,
   list_utils::head
};

use crypto::{
   types::{marketplace::prices,usd::USD},
   algos::orders::read_marketplace
};


fn usage() {
   println!("\n./bases <marketplace JSON file>");
   println!("\n\tprints the marketplace tokens and prices");
}

fn main() {
   if let Some(filename) = head(get_args()) {
      println!("Processing {}", filename);
      let markets = read_marketplace(&filename);
      print_prices("Prices", &prices(&markets));

      // bonus:
      // print_prices("USKs", &prices_usk(&markets));
   } else {
      usage();
   }
}

trait AsNum {
   fn as_num(&self) -> f32;
}

impl AsNum for USD {
   fn as_num(&self) -> f32 { self.amount }
}

impl AsNum for f32 {
   fn as_num(&self) -> f32 { *self }
}

fn print_prices<T: AsNum + Display>(header: &str, p: &HashMap<String, T>) {
   println!("\n{header}:\n");
   let mut v: Vec<_> = p.into_iter().filter(|p| p.1.as_num() > 0.0).collect();
   let mut u = v.clone();
   v.sort_by(|x,y| y.1.as_num().partial_cmp(&x.1.as_num()).unwrap());
   u.sort_by(|x,y| x.0.cmp(&y.0));
   let w = v.iter().zip(u.iter());
   fn blah<Q: Display>((a, b): &(&String, &Q)) -> String {
      format!("{a:<10} {b:>10}")
   }
   w.for_each(|(x,y)| println!("{:<40} {}", blah(x), blah(y)));
   println!("\neliding untraded tokens.");
}
