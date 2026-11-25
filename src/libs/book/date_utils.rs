// normalizes date-usage: parsing, printing, et al

use std::{ fmt, ops::Deref, str::FromStr };
use chrono::{ NaiveDate, NaiveDateTime, Local, ParseError, SubsecRound };

use super::{ err_utils::{ ErrStr, err_or }, string_utils::s };

pub fn tm_fmt() -> String { s("%Y-%m-%d %H:%M:%S") }

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Dttm(NaiveDateTime);

impl Dttm {
    /// Captures current local wall-clock time truncated to whole seconds
    pub fn now() -> Self { Self(Local::now().naive_local().trunc_subsecs(0)) }
    pub fn into_inner(self) -> NaiveDateTime { self.0 }
}

// Automatically prints as "YYYY-MM-DD HH:MM:SS" everywhere `{}` is used
impl fmt::Display for Dttm {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0.format(&tm_fmt()))
    }
}

// Lets you call any NaiveDateTime method directly on Dttm (e.g., dt.year(), dt.date())
impl Deref for Dttm {
    type Target = NaiveDateTime;
    fn deref(&self) -> &Self::Target { &self.0 }
}

impl From<NaiveDateTime> for Dttm {
    fn from(dt: NaiveDateTime) -> Self { Self(dt.trunc_subsecs(0)) }
}

impl FromStr for Dttm {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let dt = NaiveDateTime::parse_from_str(s.trim(), &tm_fmt())?;
        Ok(Self(dt.trunc_subsecs(0)))
    }
}

fn dt_fmt() -> String { s("%Y-%m-%d") }

fn err(s: &str, d: &str) -> String {
   format!("Unable to parse {s} from '{d}'")
}

pub fn parse_date(d: &str) -> ErrStr<NaiveDate> {
   err_or(NaiveDate::parse_from_str(d, &dt_fmt()), &err("date", d))
}

pub fn parse_dttm(dttm: &str) -> ErrStr<NaiveDateTime> {
   err_or(NaiveDateTime::parse_from_str(dttm, &tm_fmt()),
          &err("datetime", dttm))
}

pub fn parse_date_and<'a>(line: &'a str) -> ErrStr<(NaiveDate, &'a str)> {
   err_or(NaiveDate::parse_and_remainder(line, &dt_fmt()), &err("date", line))
}

pub fn datef(s: &str) -> NaiveDate {
   parse_date(s).expect(&format!("{s} not in date-format"))
}

pub fn date_preceeding(d: &NaiveDate) -> ErrStr<NaiveDate> {
   d.pred_opt().ok_or(format!("Unable to get date prior to {d}"))
}

pub fn yesterday() -> NaiveDate { date_preceeding(&today()).unwrap() }

pub fn tomorrow() -> NaiveDate { today().succ_opt().unwrap() }

pub fn today() -> NaiveDate { Local::now().date_naive() }

pub fn now() -> Dttm { Dttm::now() }

// ----- TESTS -------------------------------------------------------

#[cfg(test)]
#[cfg(not(tarpaulin_include))]
pub mod functional_tests {
   use super::*;
   use paste::paste;
   use crate::{ create_testing, compose, utils::resolve };

   create_testing!("date_utils");

   run_with!("parse_date", "2026-01-30", compose!(resolve)(parse_date));

   run!("dates", {
      let td = today();
      println!("Today is {td}");
      println!("Yesterday is {}", yesterday());
      println!("Tomorrow is {}", tomorrow());
   });
   run!("now", { println!("The current date-time is {}", now()); });
}

#[cfg(test)]
mod tests {
   use super::*;
   use chrono::Duration;

   #[test] fn test_parse_date_ok() {
      let mb_dt = parse_date("2026-01-31");
      assert!(mb_dt.is_ok());
   }

   #[test] fn fail_parse_date() {
      let mb_err = parse_date("adfskljjsfdlkjdsf");
      assert!(mb_err.is_err());
   }

   #[test] fn test_today() {
      let some_date = datef("2026-01-30");
      assert!(today() > some_date);
   }

   #[test] fn test_date_preceeding_ok() {
      let tday = datef("2026-03-05");
      let mb_yday = date_preceeding(&tday);
      assert!(mb_yday.is_ok());
   }

   #[test] fn test_date_preceeding() -> ErrStr<()> {
      let tday = datef("2026-03-05");
      let prior = date_preceeding(&tday)?;
      assert_eq!("2026-03-04", &format!("{prior}"));
      Ok(())
   }

   #[test] fn test_tomorrow() {
      let tday = today();
      let tmrrw = tday + Duration::days(1);
      assert_eq!(tmrrw, tomorrow());
   }

    #[test] fn test_now_returns_current_local_time() {
        let before = now();
        let actual = now();
        let after = now();

        assert!(
            actual >= before && actual <= after,
            "Expected {} to be between {} and {}",
            actual,
            before,
            after
        );
    }
}

