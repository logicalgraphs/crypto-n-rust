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

#[cfg(not(tarpaulin_include))]
pub mod functional_tests {
   use super::*;
   use crate::create_testing;

   create_testing!("parse_utils");

   fn run_parse_id() -> ErrStr<usize> { report!("parse_id", "5", parse_id) }
   fn run_parse_int() -> ErrStr<usize> {
      report!("parse_int", "123", parse_int) }
   fn run_parse_str() -> ErrStr<usize> {
      report!("parse_str", "ugga-bugga", parse_str)
   }
   fn run_parse_usd() -> ErrStr<usize> {
      report!("parse_usd", "$314.16", parse_usd)
   }
   pub fn runoff() -> ErrStr<usize> {
      let n1 = run_parse_id()?;
      let n2 = run_parse_int()?;
      let n3 = run_parse_str()?;
      let n4 = run_parse_usd()?;
      Ok(n1+n2+n3+n4)
   }
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
