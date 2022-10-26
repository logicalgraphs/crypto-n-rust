// some utils ... for some files... ya know.

// from https://stackoverflow.com/questions/30801031/read-a-file-and-get-an-array-of-strings

use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

use crate::list_utils::{ht,tail};

pub fn lines_from_file(filename: impl AsRef<Path>)
       -> Vec<String> {
    let file = File::open(filename)
          // .expect("no such file '" + filename + "'");
          // Question: how hard can the above line possibly be to compile?
          // Answer: oh, it's not hard; it's impossible.
          .expect("no such file");

       let buf = BufReader::new(file);
       buf.lines()
           .map(|l| l.expect("Could not parse line"))
           .collect()
}

pub fn extract_date_and_body(file: impl AsRef<Path>)
      -> (String, Vec<String>) {
   if let (Some(first_line), rest) =
      ht(lines_from_file(file)) {
      if let Some(date) = first_line.strip_prefix("date scraped: ") {
         (date.to_string(),tail(rest))   // skipping the blank line
      } else {
        panic!("Could not extract the date from the file.");
      }
   } else {
      panic!("File empty");
   }
}
