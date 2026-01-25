use crate::csv_utils::{CsvWriter,CsvHeader};

use super::values::Value;

// ----- INDEXED -------------------------------------------------------

pub struct Indexed<T> {
   idx: usize, 
   value: T
}

impl<T:CsvHeader> CsvHeader for Indexed<T> {
   fn header(&self) -> String { format!("ix,{}", self.value.header()) }
}

impl<T:CsvWriter> CsvWriter for Indexed<T> {
   fn as_csv(&self) -> String {
      format!("{},{}", self.idx, self.value.as_csv())
   }
   fn ncols(&self) -> usize { 1 + self.value.ncols() }
}

impl <T:Clone> Value<T> for Indexed<T> {
   fn value(&self) -> T { self.value.clone() }
}

impl <T> Indexed<T> {
   pub fn ix(&self) -> usize { self.idx }
}

// useful when enumerating over a Vec: map this fn to make an Indexed-value

pub fn mk_idx<T: Clone>(i: usize, p: &T) -> Indexed<T> {
   Indexed { idx: i, value: p.clone() }
}

pub fn mk_idx_offset<'a, T: Clone>(offset: usize)
      -> impl Fn((usize, &'a T)) -> Indexed<T> {
   move |pear: (usize, &'a T)| {
      let (i, p) = pear;
      Indexed { idx: i + offset, value: p.clone() }
   }
}

// ----- TESTS -------------------------------------------------------

#[cfg(test)]
mod tests {
   use super::*;

   #[test]
   fn test_mk_idx() {
      let ts = mk_idx(1, &2);
      assert_eq!(1, ts.ix());
      assert_eq!(2, ts.value());
   }

   #[test]
   fn test_mk_idx_offset() {
      let lst = vec![1,2,3];
      let ans: Vec<_> = lst.iter().enumerate().map(mk_idx_offset(1)).collect();
      assert_eq!(1, ans.first().unwrap().ix());
   }
}

