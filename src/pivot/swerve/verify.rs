use std::collections::HashSet;

use crate::types::{
   QuoteDict,
   Diff::{MISSING, ADDED},
   Diffs,
   Quote,
   RawPrices,
   Token,
   TokenId
};

pub fn verify(dict: &QuoteDict, prices: &RawPrices) -> Option<Diffs> {
   fn capt<V>(m: &Vec<(TokenId, V)>) -> HashSet<TokenId> {
      fn key<V>(kv: &(String, V)) -> String { kv.0.to_string() }
      m.into_iter().map(key).collect()
   }
   fn cloner<V: Clone>((k, v): (&String, &V)) -> (String, V) {
      (k.clone(), v.clone())
   }
   let v1: Vec<(TokenId, Token)> = dict.into_iter().map(cloner).collect();
   let requested = capt(&v1);
   let v2: Vec<(TokenId, Quote)> = prices.into_iter().map(cloner).collect();
   let returned = capt(&v2);

   fn diff(a: &HashSet<TokenId>, b: &HashSet<TokenId>) -> Vec<TokenId> {
      a.difference(b).map(String::to_string).collect()
   }
   let mut missing = diff(&requested, &returned);
   missing.sort();
   if missing.is_empty() {
      let mut added = diff(&returned, &requested);
      added.sort();
      if added.is_empty() {
         None
      } else {
         Some((ADDED,added))
      }
   } else {
      Some((MISSING,missing))
   }
}
