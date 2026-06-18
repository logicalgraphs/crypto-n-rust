// common utils for (de)serializing JSON

// serde_json was recommended via https://stackoverflow.com/questions/30292752/how-do-i-parse-a-json-file

use std::fmt;
use chrono::NaiveDate;

use crate::{ err_utils::{err_or,ErrStr}, string_utils::{bracket,dequote} };

extern crate serde;

use serde_json::Value;

pub fn val_str(val: &Value, idx: &str) -> String { val[idx].to_string() }

pub fn unquot(val: &Value, idx: &str) -> String {
   dequote(&val_str(val, idx))
}

pub fn val_num<T: std::str::FromStr>(val: &Value, idx: &str) -> ErrStr<T>
      where <T as std::str::FromStr>::Err: std::fmt::Debug {
   let val_str: String = val_str(val, idx);
   err_or(val_str.parse(), &format!("{idx} value is not numerical: {val_str}"))
}

pub fn val_date(val: &Value, idx: &str) -> ErrStr<NaiveDate> {
   let mut date = unquot(val, idx);
   date.truncate(10);
   err_or(NaiveDate::parse_from_str(&date, "%Y-%m-%d"),
          &format!("{date} not a parseable date!"))
}

pub trait AsJSON {
   fn as_json(&self) -> String;
}

pub fn json_list<T: AsJSON>(list: &[T]) -> String {
   let elts: Vec<String> = list.into_iter().map(AsJSON::as_json).collect();
   bracket("[]", &elts.join(",\n   "))
}

pub fn to_object<T: fmt::Display>(field_names: &str, vals: &[T]) -> String {
   fn field<P: fmt::Display>((k, v): (&str, &P)) -> String {
      format!("{k:?}: {v}")
   }
   let fields: Vec<String> =
      field_names.split_whitespace().zip(vals.iter()).map(field).collect();
   bracket("{{}}", &format!(" {} ", fields.join(", ")))
}

// ----- TESTS -------------------------------------------------------

#[cfg(test)]
#[cfg(not(tarpaulin_include))]
mod test_data {
   use super::*;
   use crate::types::tagged::{Tag,mk_tag};

   pub fn sample_json() -> ErrStr<Value> {
      let json_str = r#"
         {
            "name": "Alice Wonderkindt",
            "age": 30,
            "birthdate": "1996-04-13",
            "skills": ["Rust", "Python"]
         }
      "#;

      // Parse the string into a generic serde_json::Value
      err_or(serde_json::from_str(json_str), "Could not parse JSON value")
   }
   pub fn as_json_list() -> Vec<Tag<usize>> {
      vec![mk_tag("atoms", 117), mk_tag("quarks", 6), mk_tag("cells", 205)]
   }
}

#[cfg(test)]
#[cfg(not(tarpaulin_include))]
mod functional_tests {
   use super::*;
   use super::test_data::as_json_list;
   use paste::paste;
   use crate::create_testing;

   create_testing!("json_utils");
   run!("to_object", {
      let obj = to_object("apples bananas cranberries", &[1,2,3]);
      println!("A JSON (deserialized) object:\n\n{obj}");
   });
   run!("json_list", {
      let list = json_list(&as_json_list());
      println!("A JSON list:\n\n{list}");
   });
}

#[cfg(test)]
#[cfg(not(tarpaulin_include))]
mod tests {
   use super::*;
   use super::test_data::sample_json;
   use crate::date_utils::yesterday;

   #[test] fn test_unquot() -> ErrStr<()> {
      let json = sample_json()?;
      let name = unquot(&json, "name");
      assert_eq!("Alice Wonderkindt", &name);
      Ok(())
   }

   #[test] fn test_val_num() -> ErrStr<()> {
      let json = sample_json()?;
      let age: usize = val_num(&json, "age")?;
      assert_eq!(30, age);
      Ok(())
   }

   #[test] fn test_val_date() -> ErrStr<()> {
      let json = sample_json()?;
      let date: NaiveDate = val_date(&json, "birthdate")?;
      assert!(date < yesterday());
      Ok(())
   }
}
