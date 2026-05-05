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

pub fn date_preceeding(d: &NaiveDate) -> ErrStr<NaiveDate> {
   d.pred_opt().ok_or(format!("Unable to get date prior to {d}"))
}

pub fn yesterday() -> NaiveDate { date_preceeding(&today()).unwrap() }

pub fn tomorrow() -> NaiveDate { today().succ_opt().unwrap() }

pub fn today() -> NaiveDate {
   Local::now().date_naive()
}

// ----- TESTS -------------------------------------------------------

#[cfg(test)]
#[cfg(not(tarpaulin_include))]
pub mod functional_tests {

   use super::*;
   use paste::paste;
   use crate::{ create_testing, compose, utils::resolve };

   create_testing!("date_utils");

   run_with!("parse_date", "2026-01-30", compose!(resolve)(parse_date));

   run!("today", {
      let td = today();
      println!("Today is {td}");
      println!("Yesterday is {}", yesterday());
      println!("Tomorrow is {}", tomorrow());
   });

   run_all_functional_tests!();
}

#[cfg(test)]
mod tests {

   use super::*;
   use chrono::Duration;

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

   #[test]
   fn test_date_preceeding_ok() {
      let tday = datef("2026-03-05");
      assert!(date_preceeding(&tday).is_ok());
   }

   #[test]
   fn test_date_preceeding() -> ErrStr<()> {
      let tday = datef("2026-03-05");
      let prior = date_preceeding(&tday)?;
      assert_eq!("2026-03-04", &format!("{prior}"));
      Ok(())
   }

   #[test]
   fn test_tomorrow() {
      let tday = today();
      let tmrrw = tday + Duration::days(1);
      assert_eq!(tmrrw, tomorrow());
   }
}

