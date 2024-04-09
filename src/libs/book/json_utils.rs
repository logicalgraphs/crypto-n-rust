// common utils for (de)serializing JSON

// serde_json was recommended via https://stackoverflow.com/questions/30292752/how-do-i-parse-a-json-file

use crate::string_utils::dequote;

extern crate serde;

use serde_json::Value;

pub fn val_str(val: &Value, idx: &str) -> String {
   val[idx].to_string()
}

pub fn unquot(val: &Value, idx: &str) -> String {
   dequote(&val_str(val, idx))
}

pub fn val_num<T: std::str::FromStr>(val: &Value, idx: &str) -> T
      where <T as std::str::FromStr>::Err: std::fmt::Debug {
   let val_str: String = val_str(val, idx);
   val_str.parse().expect(&format!("{idx} value is not numerical: {val_str}"))
}

pub fn val_date(val: &Value, idx: &str) -> String {
   let mut date = unquot(val, idx);
   date.truncate(10);
   date
}

pub trait AsJSON {
   fn as_json(&self) -> String;
}

/*
impl AsJSON for Vec<T: AsJSON> {
   fn as_json(&self) -> String {
      format!("[{}]", self.into_iter().map(|j| j.as_json()).join(",\n"))
   }
}
*/
