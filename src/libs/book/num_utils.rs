use crate::err_utils::{ErrStr,err_or};

// ----- parsers -------------------------------------------------------

pub fn parse_or(n_opt: Option<&String>, default: f32) -> f32 {
   n_opt.and_then(|n| parse_num(n).ok()).unwrap_or(default)
}

pub fn parse_num(s: &str) -> ErrStr<f32> {
   parse_commaless(s)
}

// for when we wish to treat blanks (e.g.) in spreadsheets as 0.0
pub fn parse_num_or_zero(s: &str) -> ErrStr<f32> {
   if s == "" { Ok(0.0) } else { parse_num(s) }
}

pub fn parse_commaless(str: &str) -> ErrStr<f32> {
   let mut no_comma = str.to_string();
   no_comma.retain(no(','));
   err_or(no_comma.parse(), &format!("{str} is not a number"))
}

fn no(ch: char) -> impl Fn(char) -> bool {
   move |chr| chr != ch
}

// --- minimax -------------------------------------------------------

pub fn sort_f32(v: &mut Vec<f32>) {
   v.sort_by(|a,b| a.partial_cmp(&b).unwrap());
}

pub fn minimax_f32(v: &Vec<f32>) -> (Option<f32>, Option<f32>) {
   let mut srtd = v.clone();
   sort_f32(&mut srtd);
   (srtd.first().cloned(), srtd.last().cloned())
}

// ----- TESTS -------------------------------------------------------

#[cfg(not(tarpaulin_include))]
pub mod functional_tests {
   use super::*;

   pub fn s(str: &str) -> String { str.to_string() }

   fn run_parse_or() -> ErrStr<usize> {
      println!("\nnum_utils::parse_or (10.0 default) functional test\n");
      let vals: &[Option<&String>] =
         &[Some(&s("1,234.5")), Some(&s("ginger")), None];
      fn parse_or_10(n: &Option<&String>) -> f32 { parse_or(*n, 10.0) }
      let vals2: Vec<f32> = vals.into_iter().map(parse_or_10).collect();
      println!("\tThe parsed numbers from {vals:?}\n\tare {vals2:?}");
      println!("\nnum_utils::parse_or:...ok");
      Ok(1)
   }

   pub fn runoff() -> ErrStr<usize> {
      println!("\nnum_utils functional tests\n");
      let a = run_parse_or()?;
      Ok(a)
   }
}

#[cfg(test)]
mod tests {
   use super::*;
   use super::functional_tests::s;

   #[test]
   fn test_parse_num_ok() {
      let ans = parse_num("123.34");
      assert!(ans.is_ok());
   }

   #[test]
   fn fail_parse_num() {
      let ans = parse_num("ginger");
      assert!(ans.is_err());
   }

   #[test]
   fn test_parse_num() -> ErrStr<()> {
      let ans = parse_num("123.45")?;
      assert_eq!(123.45, ans);
      Ok(())
   }

   #[test]
   fn test_parse_num_commaful() -> ErrStr<()> {
      let ans = parse_num("123,456,789.01")?;
      assert_eq!(123456789.01, ans);
      Ok(())
   }

   #[test]
   fn test_parse_or_parse() {
      let ans = parse_or(Some(&s("1,234.5")), 0.0);
      assert_eq!(1234.5, ans);
   }

   #[test]
   fn test_parse_or_or() {
      let ans = parse_or(Some(&s("orange")), 5.3);
      assert_eq!(5.3, ans);
   }

   #[test]
   fn test_parse_or_none() {
      let ans = parse_or(None, 1.2);
      assert_eq!(1.2, ans);
   }
}

