use std::{
   cmp::Ordering,
   collections::HashMap
};

use book::{
   file_utils::extract_date_and_body,
   list_utils::tail,
   utils::get_args
};

use crypto::{
   rest_utils::read_market_json,
   types::books::{parse_books,prices}
};

fn usage() {
   println!("./prices <asset aliases CSV>\n");
   println!("\tOutputs assets and their price-quotes.");
}

fn main() -> Result<(), String> {
   if let Some(aliases_file) = get_args().first() {
      let aliases = load_aliases(&aliases_file);
      println!("asset,quote");
      let market = read_market_json()?;
      let books = parse_books(&market);
      let all_prices1 = prices(&books);
      let mut all_prices: Vec<_> =
         all_prices1.into_iter()
                    .map(|(k,v)| (alias(&aliases, &k), v))
                    .collect();
      fn root(s: &str) -> String {
         s.trim_matches(char::is_lowercase).to_string()
      }
      fn cmp(a: &str, b: &str) -> Ordering {
         root(a).cmp(&root(b)).then(a.len().cmp(&b.len()))
      }
      all_prices.sort_by(|(a, _), (b, _)| cmp(a, b));
      for (asset,price) in all_prices {
         println!("{asset},{price}");
      }
   } else { usage(); }
   Ok(())
}

fn alias(aliases: &HashMap<String, String>, i: &String) -> String {
   aliases.get(i).or(Some(i)).unwrap().clone()
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
