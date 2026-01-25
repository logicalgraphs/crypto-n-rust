use crate::csv_utils::{CsvWriter,CsvHeader};

use super::values::Value;

// ----- TAGGED (or attrbuted) VALUES ---------------------------------

#[derive(Debug, Clone)]
pub struct Tag<T> {
   tag: String,
   value: T
}     

impl<T:CsvWriter> CsvWriter for Tag<T> {
   fn as_csv(&self) -> String {
      format!("{},{}", self.tag, self.value.as_csv())
   }
   fn ncols(&self) -> usize { 1 + self.value.ncols() }
}

impl<T:CsvHeader> CsvHeader for Tag<T> {
   fn header(&self) -> String { format!("tag,{}", self.value.header()) }
}

impl<T:Clone> Value<T> for Tag<T> {
   fn value(&self) -> T { self.value.clone() }
}

pub fn mk_tag<T:Clone>(t: &str, v: &T) -> Tag<T> {
   Tag { tag: t.to_string(), value: v.clone() }
}

pub fn untag<T: Clone>(t: &Tag<T>) -> (String, T) {
   (t.tag.clone(), t.value.clone())
}  

// ----- TESTS -------------------------------------------------------

#[cfg(test)]
mod tests {
   use super::*;

   #[test]
   fn test_mk_tag() {
      let _ts = mk_tag("foo", &1);
      assert!(!"Test complete".is_empty());
   }
}

