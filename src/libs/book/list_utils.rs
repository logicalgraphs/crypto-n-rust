use std::{
   clone::Clone,
   cmp::PartialEq
};

// parse_nums() Influenced by the following overflows:

// https://stackoverflow.com/questions/27043268/convert-a-string-to-int
// https://stackoverflow.com/questions/23100534/how-to-sum-the-values-in-an-array-slice-or-vec-in-rust

pub fn parse_nums(strs: Vec<String>) -> Vec<f32> {
   strs.iter()
       .map(|n| n.parse().expect(&(n.to_owned() + " isn't a number")))
       .collect()
}

// list functions

pub fn ht<T: Clone>(list: Vec<T>) -> (Option<T>, Vec<T>) {
   if list.is_empty() {
      (None, list)
   } else {
      let (f, t) = list.split_at(1);
      (f.to_vec().pop(), t.to_vec())
   }
}

pub fn tail<T: Clone>(list: Vec<T>) -> Vec<T> {
   let (_, r) = ht(list);
   r
}

pub fn head<T: Clone>(list: Vec<T>) -> Option<T> {
   let (h, _) = ht(list);
   h
}

pub fn last<T: Clone>(mut list: Vec<T>) -> Option<T> {
   list.reverse();
   head(list)
}

// splits a list into lists along some element

// source: https://www.reddit.com/r/rust/comments/hgcpds/how_to_split_a_vector_by_an_entry_and_collect_all/

pub fn split<T: PartialEq>(list: Vec<T>, splitter: T) -> Vec<Vec<T>> {
   list.into_iter().fold(Vec::new(), |mut acc, x| {
        if x == splitter || acc.is_empty() {
            acc.push(Vec::new());
        }
        acc.last_mut().unwrap().push(x);
        acc
    })
}
