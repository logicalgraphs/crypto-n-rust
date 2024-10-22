// some utils ... for some files... ya know.

// from https://stackoverflow.com/questions/30801031/read-a-file-and-get-an-array-of-strings

use std::{
    fs::File,
    io::{prelude::*, BufReader}
};

use chrono::NaiveDate;

use crate::utils::{
   date_utils::parse_date,
   err_utils::{err_or,ErrStr},
   list_utils::{ht,tail},
   string_utils::parse_lines
};

pub fn lines_from_file(filename: &str) -> ErrStr<Vec<String>> {
   let file =
      err_or(File::open(filename), &format!("no such file '{filename}'"))?;
   let buf = BufReader::new(file);
   let ans = buf.lines()
      .map(|l| l.expect("Could not parse line"))
      .collect();
   Ok(ans)
}

pub fn read_file(filename: &str) -> ErrStr<String> {
   let file = lines_from_file(&filename)?;
   Ok(file.join("\n"))
}

pub fn extract_date_and_body(file: &str) -> ErrStr<(NaiveDate, Vec<String>)> {
   let lines = lines_from_file(file)?;
   if let (Some(first_line), rest) = ht(&lines) {
      if let Some(date_str) = first_line.strip_prefix("date: ") {
         let date = parse_date(&date_str)?;
         Ok((date, tail(&rest)))   // skipping the blank line
      } else {
        Err(format!("Could not extract the date from {file}."))
      }
   } else {
      Err(format!("File {file} empty"))
   }
}

pub fn parse_data<T>(f: impl Fn(String) -> ErrStr<T>, file: &str,
                     skip_header: Option<usize>) -> ErrStr<Vec<T>> {
   let lines = lines_from_file(file)?;
   parse_lines(f, &lines, skip_header)
}
