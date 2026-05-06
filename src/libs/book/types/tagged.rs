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

pub fn mk_tag<T>(t: &str, v: T) -> Tag<T> {
   Tag { tag: t.to_string(), value: v }
}

pub fn untag<T: Clone>(t: &Tag<T>) -> (String, T) {
   (t.tag.clone(), t.value.clone())
}

pub fn tag_maker<'a, T>(t: &'a str) -> impl Fn(T) -> Tag<T>  + use<'a, T> {
   move |v: T|  mk_tag(t, v)
}

// ----- TESTS -------------------------------------------------------

#[cfg(test)]
#[cfg(not(tarpaulin_include))]
pub mod functional_tests {
   use super::*;

   use paste::paste;
   use crate::{
      create_testing,
      compose,
      err_utils::ErrStr,
      utils::{debug,composer,deref}
   };

   create_testing!("types::tagged");

   run_with!("tag_maker", 9,
             composer(deref(CsvWriter::as_csv), tag_maker("six")));
   run_with!("untag", &mk_tag("foo", "quux"), compose!(debug)(untag));
   run!("mk_tag", {
      let t = mk_tag("id", 7);
      println!("tagged 7 is {}", t.as_csv())
   });
}

