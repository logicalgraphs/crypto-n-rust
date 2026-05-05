use super::{ currency::usd::USD, err_utils::{ErrStr,err_or} };

pub fn parse_id(s: &str) -> ErrStr<usize> {
   err_or(s.parse(), &format!("{s} is not an Id-type"))
}

pub fn parse_int(s: &str) -> ErrStr<i32> {
   err_or(s.parse(), &format!("{s} is not an int"))
}

pub fn parse_str(s: &str) -> ErrStr<String> {
   Ok(s.to_string())
}

pub fn parse_usd(s: &str) -> ErrStr<USD> {
   err_or(s.parse(), &format!("Cannot parse USD from {s}"))
}

pub fn parse_nums(strs: Vec<String>) -> Vec<f32> {
   strs.into_iter().map(|n| n.parse().expect(&format!("'{n}' NaN"))).collect()
}

// ----- TESTS -------------------------------------------------------

#[cfg(test)]
#[cfg(not(tarpaulin_include))]
pub mod functional_tests {
   use super::*;
   use paste::paste;
   use crate::{ create_testing, compose, utils::resolve };

   create_testing!("parse_utils");

   run_with!("parse_id", "5", compose!(resolve)(parse_id));
   run_with!("parse_int", "123", compose!(resolve)(parse_int));
   run_with!("parse_str", "ugga-bugga", compose!(resolve)(parse_str));
   run_with!("parse_usd", "$314.16", compose!(resolve)(parse_usd));

   run_all_functional_tests!();
}

#[cfg(test)]
mod tests {
   use super::*;

   #[test]
   fn test_parse_id_ok() {
      let ans = parse_id("123");
      assert!(ans.is_ok());
   }

   #[test]
   fn fail_parse_id() {
      let ans = parse_id("orange");
      assert!(ans.is_err());
   }

   #[test]
   fn test_parse_id() -> ErrStr<()> {
      let ans = parse_id("123")?;
      assert_eq!(123, ans);
      Ok(())
   }

   #[test]
   fn test_parse_int_ok() {
      let ans = parse_int("-123");
      assert!(ans.is_ok());
   }

   #[test]
   fn fail_parse_int() {
      let ans = parse_int("orange");
      assert!(ans.is_err());
   }

   #[test]
   fn test_parse_int() -> ErrStr<()> {
      let ans = parse_int("123")?;
      assert_eq!(123, ans);
      Ok(())
   }

   #[test]
   fn test_parse_str_ok() {
      let ans = parse_str("ugga-bugga");
      assert!(ans.is_ok());
   }

   #[test]
   fn test_parse_str() -> ErrStr<()> {
      let ans = parse_str("ugga-bugga")?;
      assert_eq!("ugga-bugga", &ans);
      Ok(())
   }

   #[test]
   fn test_parse_usd_ok() {
      let ans = parse_usd("-$123.45");
      assert!(ans.is_ok());
   }

   #[test]
   fn fail_parse_usd() {
      let ans = parse_usd("orange");
      assert!(ans.is_err());
   }

   #[test]
   fn test_parse_usd() -> ErrStr<()> {
      let ans = parse_usd("-$123.45")?;
      assert_eq!(-123.45, ans.amount);
      Ok(())
   }
}
