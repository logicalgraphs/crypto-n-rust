use chrono::NaiveDate;

use crate::csv_utils::{CsvWriter,CsvHeader};

use super::values::Value;

// ----- STAMPED -------------------------------------------------------

#[derive(Debug,Clone)]
pub struct Stamped<T> {
   date: NaiveDate,
   value: T
}

pub fn stamp<T: Clone>(date: &NaiveDate, pack: &T) -> Stamped<T> {
   Stamped { date: date.clone(), value: pack.clone() }
}

impl <T> Stamped<T> {
   pub fn date(&self) -> NaiveDate { self.date.clone() }
}

impl<T:CsvHeader> CsvHeader for Stamped<T> {
   fn header(&self) -> String { format!("date,{}", self.value.header()) }
}

impl<T:CsvWriter> CsvWriter for Stamped<T> {
   fn as_csv(&self) -> String {
      format!("{},{}", self.date, self.value.as_csv())
   }
   fn ncols(&self) -> usize { 1 + self.value.ncols() }
}

impl <T:Clone> Value<T> for Stamped<T> {
   fn value(&self) -> T { self.value.clone() }
}

// ----- TESTS -------------------------------------------------------

#[cfg(test)]
mod tests {
   use super::*;
   use crate::date_utils::today;

   #[test]
   fn test_stamped() {
      let _ts = stamp(&today(), &3);
      assert!(!"Test complete".is_empty());
   }
}

