// normalizes date-usage: parsing, printing, et al

use chrono::NaiveDate;

use crate::err_utils::{err_or,ErrStr};

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
