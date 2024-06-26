// common utils for (de)serializing JSON

// serde_json was recommended via https://stackoverflow.com/questions/30292752/how-do-i-parse-a-json-file

use chrono::NaiveDate;

use crate::{
   err_utils::{err_or,ErrStr},
   string_utils::{bracket,dequote}
};

extern crate serde;

use serde_json::Value;

pub fn val_str(val: &Value, idx: &str) -> String {
   val[idx].to_string()
}

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

pub fn json_list<T: AsJSON>(list: &Vec<T>) -> String {
   let elts: Vec<String> = list.into_iter().map(AsJSON::as_json).collect();
   bracket("[]", &elts.join(",\n   "))
}

pub fn to_object(field_names: &str, vals: &[String]) -> String {
   fn field((k, v): (&str, &String)) -> String { format!("{k}: {v}") }
   let fields: Vec<String> =
      field_names.split_whitespace().zip(vals.iter()).map(field).collect();
   bracket("{{}}", &format!(" {} ", fields.join(", ")))
}
