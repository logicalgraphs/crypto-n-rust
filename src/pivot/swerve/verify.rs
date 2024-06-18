use std::collections::{HashMap,HashSet};

use crate::types::{
   Dict,
   Diff::{MISSING, ADDED},
   Diffs,
   RawPrices,
   TokenId
};

pub fn verify(dict: &Dict, prices: &RawPrices) -> Option<Diffs> {

   fn capt<V>(m: &HashMap<TokenId, V>) -> HashSet<TokenId> {
      m.keys().map(String::to_string).collect()
   }
   let requested = capt(&dict);
   let returned = capt(&prices);

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
