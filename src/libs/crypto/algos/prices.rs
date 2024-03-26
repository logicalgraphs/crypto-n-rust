use std::cmp::Ordering;

use crate::types::{
   books::Prices,
   usd::USD
};

type Pair = (String, USD);
type PairRef<'a> = (&'a String, &'a USD);

pub fn merge_prices(new_prices: &Prices, portfolio: &Prices) -> Prices {
   fn merge_price<'a>(news: &'a Prices) -> impl Fn(PairRef<'a>) -> Pair + 'a {
      |(tok,pric)| (tok.clone(),news.get(tok).unwrap_or(pric).clone())
   }
   portfolio.into_iter().map(merge_price(new_prices)).collect()
}

pub fn print_sorted_prices(prices: &Prices) {
   let mut all_prices: Vec<_> = prices.into_iter().collect();
   fn root(s: &str) -> String {
      s.trim_matches(char::is_lowercase).to_string()
   }
   fn cmp(a: &str, b: &str) -> Ordering {
      root(a).cmp(&root(b)).then(a.len().cmp(&b.len()))
   }
   all_prices.sort_by(|(a, _), (b, _)| cmp(a, b));
   println!("token\tprice");
   all_prices.into_iter().for_each(|(asst,pric)| println!("{asst}\t{pric}"));
}
