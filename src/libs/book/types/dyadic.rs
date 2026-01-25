
use crate::csv_utils::{CsvWriter,CsvHeader};

use super::values::Value;

#[derive(Debug, Clone)] 
pub struct Dyad<T> {
   pair: (String, String),
   value: T
}

impl<T:CsvWriter> CsvWriter for Dyad<T> {
   fn as_csv(&self) -> String {
      let p = &self.pair;
      format!("{},{},{}", p.0, p.1, self.value.as_csv())
   }
   fn ncols(&self) -> usize { 2 + self.value.ncols() }
}

impl<T:CsvHeader> CsvHeader for Dyad<T> {
   fn header(&self) -> String {
      format!("primary_key,secondary_key,{}", self.value.header())
   }
}

impl<T:Clone> Value<T> for Dyad<T> {
   fn value(&self) -> T { self.value.clone() }
}

pub fn mk_dyad<T:Clone>(pri: &str, snd: &str, v: T) -> Dyad<T> {
   Dyad { pair: (pri.to_string(), snd.to_string()), value: v.clone() }
}

pub fn unpair<T: Clone>(d: &Dyad<T>) -> ((String, String), T) {
   (d.pair.clone(), d.value.clone())
}     

// ----- TESTS -------------------------------------------------------

#[cfg(test)]
mod tests {
   use super::*;

   #[test]
   fn test_mk_dyad() {
      let _ts = mk_dyad("foo", "bar", "bax");
      assert!(!"Test complete".is_empty());
   }
}

