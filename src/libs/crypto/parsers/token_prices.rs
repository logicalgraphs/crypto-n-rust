use std::collections::HashMap;

use book::{
   file_utils::extract_date_and_body,
   list_utils::tail
};

use crate::types::{
   books::Prices,
   usd::USD
};

pub fn read_prices(file: &str) -> Prices {
   let (_date, lines) = extract_date_and_body(file);
   let mut ans = HashMap::new();

   for line in tail(&lines) {
      if let [asset, monay] = line.split("\t").collect::<Vec<_>>().as_slice() {
         let quot: USD = monay.parse()
                  .expect(&format!("Could not parse {monay} to USD"));
         ans.insert(asset.to_string(),quot);
      } else { panic!("Unparseable line in prices: {line}") }
   }
   ans
}
