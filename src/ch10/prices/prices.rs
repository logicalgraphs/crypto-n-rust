use std::collections::HashMap;

use book::{
   file_utils::extract_date_and_body,
   list_utils::tail,
   utils::get_args
};
use crypto::types::books::{load_books,prices};

fn usage() {
   println!("./prices <marketplace JSON> <asset aliases CSV>\n");
   println!("Outputs assets and their price-quotes.");
}

fn main() {
   if let [market, aliases_file] = get_args().as_slice() {
      let aliases = load_aliases(&aliases_file);
      println!("asset,quote");
      for (asset,price) in prices(&load_books(&market)) {
         println!("{},{price}", alias(&aliases, &asset));
      }
   } else { usage(); }
}

fn alias(aliases: &HashMap<String, String>, i: &str) -> String {
   (if let Some(ali) = aliases.get(i) { ali } else { i }).to_string()
}

fn load_aliases(file: &str) -> HashMap<String, String> {
   let mut ans = HashMap::new();
   let (date, lines) = extract_date_and_body(file);

   println!("date: {date}\n");  // outputting prices as-of when.

   for alias in tail(&lines) {
      if let [id,name] = alias.split(",").collect::<Vec<_>>().as_slice() {
        ans.insert(id.to_string(), name.to_string());
      } else { panic!("Unable to parse alias: {alias}") }
   }
   ans
}
