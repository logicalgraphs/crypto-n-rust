use std::slice::Iter;

use crypto::types::usd::USD;

use crate::pairs::Pair;

// An InfArr ('infinite array') just keeps going and going, ...
// When iterated, it iterates over the elements of `basis` then
// when exhausted, the default value of T is returned forever more.

pub struct InfArr<T> {
   basis: Vec<T>
}

pub fn mk_inf<T: Clone>(v: &Vec<T>) -> InfArr<T> {
   InfArr { basis: v.clone() }
}

pub struct InfArrIter<'a, T> {
   itr: Iter<'a, T>
}

impl<T> InfArr<T> {
   pub fn iter(&self) -> InfArrIter<'_, T> {
      InfArrIter { itr: self.basis.iter() }
   }
}

// Basic implementation over Pair<USD> vectors.

impl<'a> Iterator for InfArrIter<'a, Pair<USD>> {
   type Item = Pair<USD>;
   fn next(&mut self) -> Option<Self::Item> {
      let mut ans = Pair::default();
      if let Some(a) = self.itr.next() {
         ans = a.clone()
      }
      Some(ans)
   }
}
