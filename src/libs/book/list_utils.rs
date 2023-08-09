use std::{
   clone::Clone,
   cmp::PartialEq,
   collections::HashMap,
   hash::Hash
};

// parse_nums() Influenced by the following overflows:

// https://stackoverflow.com/questions/27043268/convert-a-string-to-int
// https://stackoverflow.com/questions/23100534/how-to-sum-the-values-in-an-array-slice-or-vec-in-rust

pub fn parse_nums(strs: Vec<String>) -> Vec<f32> {
   parse_nums_res(strs).iter().map(|n| n.clone().expect("what?")).collect()
}

pub fn parse_nums_opt(strs: Vec<String>) -> Vec<f32> {
   let mut ans: Vec<f32> = Vec::new();
   for x in parse_nums_res(strs) {
      if let Ok(n) = x {
         ans.push(n);
      }
   }
   ans
}

pub fn parse_nums_res(strs: Vec<String>) -> Vec<Result<f32, String>> {
   strs.iter().map(|n| match n.parse() {
      Ok(x) => Ok (x),
      Err(_) => {
        let msg = String::from(&(n.to_owned() + " isn't a number"));
        Err(msg)
      }}).collect()
}

// list functions

pub fn ht<T: Clone>(list: &Vec<T>) -> (Option<T>, Vec<T>) {
   if list.is_empty() {
      (None, list.clone())
   } else {
      let (f, t) = list.split_at(1);
      (f.to_vec().pop(), t.to_vec())
   }
}

pub fn tail<T: Clone>(list: &Vec<T>) -> Vec<T> {
   let (_, r) = ht(list);
   r
}

pub fn head<T: Clone>(list: &Vec<T>) -> Option<T> {
   let (h, _) = ht(list);
   h
}

pub fn last<T: Clone>(mut list: Vec<T>) -> Option<T> {
   list.reverse();
   head(&list)
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

// makes a HashMap from a list, given a key-(extraction)-function

pub fn assoc_list<K: Eq + Hash, T: Clone>(list: Vec<T>, f: impl Fn(T) -> K)
 -> HashMap<K, T> {
   list.into_iter().map(|t| (f(t.clone()), t)).collect()
}