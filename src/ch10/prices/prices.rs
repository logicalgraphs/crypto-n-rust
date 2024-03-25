use std::cmp::Ordering;

use crypto::types::books::prices_with_aliases;

fn main() {
   let all_prices1 = prices_with_aliases();
   let mut all_prices: Vec<_> = all_prices1.into_iter().collect();
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
}
