// common utils for deserializing JSON

extern crate serde;

use serde_json::Value;

pub fn val_str(val: &Value, idx: &String) -> String {
   val[idx].to_string()
}

pub fn val_num<T: std::str::FromStr>(val: &Value, idx: String) -> T
      where <T as std::str::FromStr>::Err: std::fmt::Debug {
   let val_str: String = val_str(val, &idx);
   val_str.parse().expect(&idx)
}

pub fn val_date(val: &Value, idx: &String) -> String {
   let mut date = dequote(val_str(val, idx));
   date.truncate(10);
   date
}

pub fn dequote(mut str: String) -> String {
   str.pop();
   str.remove(0);
   str
}
