use chrono::NaiveDate;

use crate::csv_utils::CsvWriter;

// ----- STAMPED -------------------------------------------------------

#[derive(Debug,Clone)]
pub struct Stamped<T> {
   pub date: NaiveDate,
   pub pack: T
}

pub fn stamp<T: Clone>(date: &NaiveDate, pack: &T) -> Stamped<T> {
   Stamped { date: date.clone(), pack: pack.clone() }
}

impl<T:CsvWriter> CsvWriter for Stamped<T> {
   fn as_csv(&self) -> String {
      format!("{},{}", self.date, self.pack.as_csv())
   }
   fn ncols(&self) -> usize { 1 + self.pack.ncols() }
}

// ----- INDEXED -------------------------------------------------------

pub struct Indexed<T> {
   pub idx: usize, 
   pub pack: T
}

impl<T:CsvWriter> CsvWriter for Indexed<T> {
   fn as_csv(&self) -> String {
      format!("{},{}", self.idx, self.pack.as_csv())
   }
   fn ncols(&self) -> usize { 1 + self.pack.ncols() }
}

// useful when enumerating over a Vec: map this fn to make an Indexed-value

pub fn mk_idx<T: Clone>(i: usize, p: &T) -> Indexed<T> {
   Indexed { idx: i, pack: p.clone() }
}

pub fn mk_idx_offset<T: Clone>(pear: (usize, &T)) -> Indexed<T> {
   let (i, p) = pear;
   Indexed { idx: i+1, pack: p.clone() }
}
