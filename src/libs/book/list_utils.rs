use std::{
   clone::Clone,
   fmt::{Debug,Formatter, Result as Fresult},  // y'all can thank the Dylan
                                               // programming language for
                                               // item-renaming.
   slice::Iter
};
use super::err_utils::ErrStr;

// ----- infinite lists --------------------------------------------------

#[derive(Clone)]
pub struct InfiniteList<T> {
   acid: T,
   base: Vec<T>
}

pub fn mk_inf<T: Clone>(v: &[T], d: T) -> InfiniteList<T> {
   InfiniteList { acid: d, base: v.to_vec() }
}

pub fn mk_cycle<T: Clone>(a: &T) -> InfiniteList<T> {
   InfiniteList { acid: a.clone(), base: [].to_vec() }
}

pub struct InfListItr<'a, T> {
   itr: Iter<'a, T>,
   def: T
}

impl <T:Clone> InfiniteList<T> {
   pub fn iter(&self) -> InfListItr<'_, T> {
      InfListItr { itr: self.base.iter(), def: self.acid.clone() }
   }
}

impl<'a, T:Clone> Iterator for InfListItr<'a, T> {
   type Item = T;
   fn next(&mut self) -> Option<Self::Item> {
      Some((if let Some(a) = self.itr.next() { a } else { &self.def }).clone())
   }
}

impl<T:Debug> Debug for InfiniteList<T> {
   fn fmt(&self, f: &mut Formatter<'_>) -> Fresult {
      fn just_write(f: &mut Formatter<'_>, s: &str) {
         fn ki() { }  // from combinatorics
         match write!(f, "{}", s) {
            Ok(_) => ki(),
            Err(x) => panic!("Err'd on format {x:?}!!!")
         };
      }
      just_write(f, &if !self.base.is_empty() {
         let bits = init(&format!("{:?}", self.base).into_bytes());
         let chonk = String::from_utf8(bits).unwrap();
         format!("{chonk}, ")
      } else { "[".to_string() });
      let def: &T = &self.acid;
      write!(f, "{def:?}, {def:?}, {def:?}, ...]")
   }
}

pub fn parse_nums(strs: Vec<String>) -> Vec<f32> {
   strs.into_iter().map(|n| n.parse().expect(&format!("'{n}' NaN"))).collect()
}

// ----- list functions --------------------------------------------------

pub fn ht<T: Clone>(list: &[T]) -> (Option<T>, Vec<T>) {
   let listy = list.to_vec();
   if listy.is_empty() {
      (None, listy)
   } else {
      let (f, t) = listy.split_at(1);
      (f.to_vec().pop(), t.to_vec())
   }
}

pub fn tail<T: Clone>(list: &[T]) -> Vec<T> {
   let (_, r) = ht(list);
   r
}

pub fn head<T: Clone>(list: &[T]) -> Option<T> {
   let (h, _) = ht(list);
   h
}

pub fn first_last<T: Clone>(v: &[T]) -> (Option<T>, Option<T>) {
   (v.first().cloned(), v.last().cloned())
}

pub fn init<T: Clone>(list: &[T]) -> Vec<T> {
   let v1: Vec<&T> = list.iter().rev().collect();
   let mut ans: Vec<T> = Vec::new();
   for t in tail(&v1).into_iter().rev() {
      ans.push(t.clone());
   }
   ans
}

pub fn postpend<T: Clone>(list: &[T], t: T) -> Vec<T> {
   [list, &[t]].concat().to_vec()
}

pub fn filter_map_or<D,R>(f: impl Fn(D) -> ErrStr<R>,
                        v: Vec<D>) -> ErrStr<Vec<R>> {
   let mut ans: Vec<R> = Vec::new();
   for elt in v {
      let eh = f(elt)?;
      ans.push(eh);
   }
   Ok(ans) 
}  

// ----- TESTS -------------------------------------------------------

#[cfg(not(tarpaulin_include))]
pub mod functional_tests {
   use super::*;
   use crate::{
      test_utils::{preamble,reporter,bind},
      utils::id
   };

   fn module() -> String { "list_utils".to_string() }
   fn run_mk_inf() -> ErrStr<usize> {
      reporter(module())("mk_inf", mk_inf(&[3,1,4,1,5,9], 9), bind(id))
   }

   pub fn runoff() -> ErrStr<usize> {
      preamble(&module());
      let a = run_mk_inf()?;
      Ok(a)
   }
}

#[cfg(test)]
mod tests {
   use super::*;

   fn one() -> Vec<i32> { vec![1] }
   fn one_two() -> Vec<i32> { vec![1,2] }

   #[test]
   fn test_ht_empty_list() {
      let list: Vec<usize> = Vec::new();
      let (h, t) = ht(&list);
      assert_eq!(None, h);
      assert!(t.is_empty());
   }

   #[test]
   fn test_ht_uno() {
      let (h,t) = ht(&one());
      assert_eq!(Some(1), h);
      assert!(t.is_empty())
   }

   #[test]
   fn test_ht_uno_dos() {
      let (h,t) = ht(&one_two());
      assert_eq!(Some(1), h);
      assert_eq!(vec![2], t);
   }

   #[test]
   fn test_postpend() {
      let vec = one();
      let res = postpend(&vec, 2);
      assert_eq!(one_two(), res);
   }

   #[test]
   fn test_head_none() {
      let empt: Vec<i32> = Vec::new();
      assert_eq!(None, head(&empt));
   }

   #[test]
   fn test_head_some_one() {
      assert_eq!(Some(1), head(&one()));
   }

   #[test]
   fn test_head_some_one_too() {
      assert_eq!(Some(1), head(&one_two()));
   }

   #[test]
   fn test_tail_empty() {
      assert!(tail(&one()).is_empty());
   }

   #[test]
   fn test_tail_too() {
      assert_eq!(vec![2], tail(&one_two()));
   }
}
