use std::{
   clone::Clone,
   fmt::{Debug,Formatter, Result as Fresult},  // y'all can thank the Dylan
                                               // programming language for
                                               // item-renaming.
   slice::Iter
};
use super::err_utils::ErrStr;

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

pub fn take<T: Clone>(n: usize, v: &[T]) -> Vec<T> {
   v.into_iter().take(n).cloned().collect()
}

pub fn first_last<T: Clone>(v: &[T]) -> (Option<T>, Option<T>) {
   (v.first().cloned(), v.last().cloned())
}

// like a tuple, ... but a list
pub fn fst_snd<T: Clone + Debug>(list: &[T]) -> ErrStr<(T, T)> {
   fn firstly<U: Clone + Debug>(lst: &[U]) -> ErrStr<(U, Vec<U>)> {
      let a = lst.first().ok_or(format!("Cannot first() this list {lst:?}"))?;
      Ok((a.clone(), tail(lst)))
   }
   let (a, rest) = firstly(list)?;
   let (b, _) = firstly(&rest)?;
   Ok((a, b))
}

pub fn init<T: Clone>(list: &[T]) -> Vec<T> {
   let v1: Vec<&T> = list.into_iter().rev().collect();
   tail(&v1).into_iter().rev().cloned().collect()
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

pub fn parse_nums(strs: &[String]) -> Vec<f32> {
   strs.into_iter().map(|n| n.parse().expect(&format!("'{n}' NaN"))).collect()
}

// ----- TESTS -------------------------------------------------------

#[cfg(test)]
#[cfg(not(tarpaulin_include))]
pub mod functional_tests {
   use super::*;
   use paste::paste;
   use crate::{ create_testing, utils::debug };

   create_testing!("list_utils");
   run_with!("mk_inf", mk_inf(&[3,1,4,1,5,9], 9), debug);
}

#[cfg(test)]
#[cfg(not(tarpaulin_include))]
mod tests {
   use super::*;
   use crate::string_utils::s;

   fn none() -> Vec<i32> { Vec::new() }
   fn one() -> Vec<i32> { vec![1] }
   fn one_two() -> Vec<i32> { vec![1,2] }
   fn ten() -> Vec<i32> { vec![1,2,3,4,5,6,7,8,9,10] }

   #[test] fn test_ht_empty_list() {
      let (h, t) = ht(&none());
      assert_eq!(None, h);
      assert!(t.is_empty());
   }

   #[test] fn test_ht_uno() {
      let (h,t) = ht(&one());
      assert_eq!(Some(1), h);
      assert!(t.is_empty())
   }

   #[test] fn test_ht_uno_dos() {
      let (h,t) = ht(&one_two());
      assert_eq!(Some(1), h);
      assert_eq!(vec![2], t);
   }

   #[test] fn test_postpend() {
      let vec = one();
      let res = postpend(&vec, 2);
      assert_eq!(one_two(), res);
   }

   #[test] fn test_head_none() {
      let empt: Vec<i32> = Vec::new();
      assert_eq!(None, head(&empt));
   }

   #[test] fn test_head_some_one() { assert_eq!(Some(1), head(&one())); }
   #[test]fn test_head_some_one_too() { assert_eq!(Some(1), head(&one_two())); }
   #[test] fn test_tail_empty() { assert!(tail(&one()).is_empty()); }
   #[test] fn test_tail_too() { assert_eq!(vec![2], tail(&one_two())); }
   #[test] fn test_take_5() {
      assert_eq!(10, ten().len());
      assert_eq!(5, take(5, &ten()).len());
   }

   #[test] fn test_first_last_on_ten() {
      let (o, t) = first_last(&ten());
      assert_eq!(Some(1), o);
      assert_eq!(Some(10), t);
   }
   #[test] fn test_first_last_on_none() {
      let nein = first_last(&none());
      assert_eq!((None, None), nein);
   }
   #[test] fn test_fst_snd_on_ten() -> ErrStr<()> {
      let (o, t) = fst_snd(&ten())?;
      assert_eq!(1, o);
      assert_eq!(2, t);
      Ok(())
   }
   #[test] fn fail_fst_snd_on_one() {
      let ans = fst_snd(&one());
      assert!(ans.is_err());
   }
   #[test] fn test_init_one_two() { assert_eq!(vec![1], init(&one_two())); }
   #[test] fn test_init_none() { assert_eq!(none(), init(&none())); }
   #[test] fn test_init_ten() {
      assert_eq!(vec![1,2,3,4,5,6,7,8,9], init(&ten()));
   }
   #[test] fn test_postpend_none() { assert_eq!(one(), postpend(&none(), 1)); }
   #[test] fn test_postpend_three() {
      assert_eq!(vec![1,2,3], postpend(&one_two(), 3));
   }

   fn uno() -> impl Fn(i32) -> ErrStr<String> {
      move |x: i32|
         match x {
            1 => Ok(s("one")),
            y => Err(format!("{y} is not one"))
         }
   }
   #[test] fn fail_filter_map_or() { 
      let ans = filter_map_or(uno(), ten());
      assert!(ans.is_err());
   }
   #[test] fn test_filter_map_or_one() -> ErrStr<()> {
      assert_eq!(vec![s("one")], filter_map_or(uno(), one())?);
      Ok(())
   }
   #[test] fn test_filter_map_or_ok() {
      assert!(filter_map_or(uno(), none()).is_ok());
   }
}
