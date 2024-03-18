use std::{
   clone::Clone,
   fmt::{Debug,Formatter, Result as Fresult},  // y'all can thank the Dylan
                                               // programming language for
                                               // item-renaming.
   slice::Iter
};

// ----- infinite lists --------------------------------------------------

pub struct InfiniteList<T> {
   acid: T,
   base: Vec<T>
}

pub fn mk_inf<T: Clone>(v: &Vec<T>, d: T) -> InfiniteList<T> {
   InfiniteList { acid: d, base: v.clone() }
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

pub fn first_last<T: Clone>(v: &Vec<T>) -> (Option<T>, Option<T>) {
   (v.first().cloned(), v.last().cloned())
}

pub fn init<T: Clone>(list: &Vec<T>) -> Vec<T> {
   let v1: Vec<&T> = list.iter().rev().collect();
   let mut ans: Vec<T> = Vec::new();
   for t in tail(&v1).into_iter().rev() {
      ans.push(t.clone());
   }
   ans
}
