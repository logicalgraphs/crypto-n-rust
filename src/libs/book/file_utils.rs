// some utils ... for some files... ya know.

// from https://stackoverflow.com/questions/30801031/read-a-file-and-get-an-array-of-strings

use std::{
    fs::File,
    io::{prelude::*, BufReader},
};

use crate::{
   list_utils::{ht,tail},
   string_utils::parse_lines
};

pub fn lines_from_file(filename: &str) -> Vec<String> {
   let file = File::open(filename)
               .expect(&format!("no such file '{filename}'"));

   let buf = BufReader::new(file);
   buf.lines()
      .map(|l| l.expect("Could not parse line"))
      .collect()
}

pub fn read_file(filename: &str) -> String {
   lines_from_file(&filename).join("\n")
}

pub fn extract_date_and_body(file: &str) -> (String, Vec<String>) {
   if let (Some(first_line), rest) =
      ht(&lines_from_file(file)) {
      if let Some(date) = first_line.strip_prefix("date: ") {
         (date.to_string(),tail(&rest))   // skipping the blank line
      } else {
        panic!("Could not extract the date from the file.");
      }
   } else {
      panic!("File empty");
   }
}

pub fn parse_data<T>(f: impl Fn(String) -> Result<T, String>, file: &str,
                     skip_header: Option<usize>) -> Result<Vec<T>, String> {
   let lines = lines_from_file(file);
   parse_lines(f, &lines, skip_header)
}
