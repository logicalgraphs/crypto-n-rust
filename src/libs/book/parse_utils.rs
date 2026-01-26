use super::err_utils::{ErrStr,err_or};

pub fn parse_id(s: &str) -> ErrStr<usize> {
   err_or(s.parse(), &format!("{s} is not an Id-type"))
}

pub fn parse_int(s: &str) -> ErrStr<i32> {
   err_or(s.parse(), &format!("{s} is not an int"))
}

pub fn parse_str(s: &str) -> ErrStr<String> {
   Ok(s.to_string())
}

