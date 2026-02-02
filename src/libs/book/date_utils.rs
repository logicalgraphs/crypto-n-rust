// normalizes date-usage: parsing, printing, et al

use chrono::{NaiveDate,Local};

use super::err_utils::{err_or,ErrStr};

fn dt_fmt() -> String { "%Y-%m-%d".to_string() }

fn err(s: &str) -> String { format!("Unable to parse date from '{s}'") }

pub fn parse_date(d: &str) -> ErrStr<NaiveDate> {
   err_or(NaiveDate::parse_from_str(d, &dt_fmt()), &err(d))
}

pub fn parse_date_and<'a>(line: &'a str) -> ErrStr<(NaiveDate, &'a str)> {
   err_or(NaiveDate::parse_and_remainder(line, &dt_fmt()), &err(line))
}

pub fn datef(s: &str) -> NaiveDate {
   parse_date(s).expect(&format!("{s} not in date-format"))
}

pub fn today() -> NaiveDate {
   Local::now().date_naive()
}

pub mod functional_tests {

   use super::*;

   use crate::test_utils::{mk_tests,collate_results,same,Thunk::E};

   fn run_parse_date() -> ErrStr<usize> {
      println!("\nparse_date functional test\n");
      let dt_str = "2026-01-30";
      let dt = parse_date(dt_str);
      println!("Parsing date {}; result: {:?}", dt_str, dt);
      match dt { Ok(_) => Ok(1), Err(str) => Err(str) }
   }

   fn run_today() -> ErrStr<usize> {
      let td = today();
      let td_str = format!("{td}");
      println!("\ntoday functional test\n");
      println!("Today is {td}");
      same(td, datef(&td_str))
   }

   pub fn runoff() -> ErrStr<usize> {
      collate_results("date_utils",
         &mut mk_tests("run_parse_date run_today",
                       vec![E(run_parse_date), E(run_today)]))
   }
}

#[cfg(test)]
mod tests {

   use super::*;

   #[test]
   fn test_parse_date_ok() {
      let mb_dt = parse_date("2026-01-31");
      assert!(mb_dt.is_ok());
   }

   #[test]
   fn fail_parse_date() {
      let mb_err = parse_date("adfskljjsfdlkjdsf");
      assert!(mb_err.is_err());
   }

   #[test]
   fn test_today() {
      let some_date = datef("2026-01-30");
      assert!(today() > some_date);
   }
}

