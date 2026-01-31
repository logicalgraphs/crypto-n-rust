// some utils ... for some files... ya know.

// from https://stackoverflow.com/questions/30801031/read-a-file-and-get-an-array-of-strings

use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::{Path,PathBuf}
};

use chrono::NaiveDate;
use ignore::WalkBuilder;

use super::{
   date_utils::parse_date,
   err_utils::{err_or,ErrStr},
   list_utils::{ht,tail},
   string_utils::parse_lines,
   tuple_utils::Partition
};

fn dir_filter(dirs: Vec<PathBuf>, dir: &str) -> Vec<PathBuf> {
   let mut normal_dirs = Vec::new();
   let pwd = Path::new(dir).to_owned();
   for path in dirs {
      let mb_file = path.file_name();
      if let Some(file) = mb_file {
         let f = file.to_string_lossy().to_string();
         if !(path == pwd) && !f.starts_with(".") {
            normal_dirs.push(path);
         }
      }
   }
   normal_dirs
}

pub fn subdirs(dir: &str) -> Vec<PathBuf> {
   let mut binding = WalkBuilder::new(dir);
   let mut bldr = binding.max_depth(Some(1));
   let (dirs, _files) = dirs_files_special_dirs(&mut bldr);
   dir_filter(dirs, dir)
}

pub fn dirs_files(dir: &str) -> Partition<PathBuf> {
   let mut bldr = WalkBuilder::new(dir);
   let (dirs, files) = dirs_files_special_dirs(&mut bldr);
   let normal_dirs = dir_filter(dirs, dir);
   (normal_dirs, files)
}

pub fn dirs_files_special_dirs(walk_builder: &mut WalkBuilder)
      -> Partition<PathBuf> {
   let walk = walk_builder.build();
   walk.into_iter()
       .filter_map(|e| e.ok())
       .map(|e| e.path().to_owned())
       .partition(|p| p.is_dir())
}

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

#[cfg(test)]
mod tests {
   use super::*;

   #[test]
   fn test_dirs_files() {
      let ans = dirs_files(".");
      let (dirs, files) = &ans;
      assert_eq!(4, dirs.len());
      // assert_eq!("bar", &format!("{files:?}")); // to see the files
      assert!(files.len() > 10);
   }

   #[test]
   fn test_read_file_ok() {
      let testing_with_this_very_file = "file_utils.rs";
      let file = read_file(testing_with_this_very_file);
      assert!(file.is_ok());
   }

   #[test]
   fn fail_read_file() {
      let fail_file = read_file("adfkljaskldf dlksdakj");
      assert!(fail_file.is_err());
   }

   #[test]
   fn test_line_from_file() -> ErrStr<()> {
      let lines = lines_from_file("file_utils.rs")?;
      assert!(lines.len() > 5);
      Ok(())
   }

   #[test]
   fn test_subdirs() {
      let ans = subdirs(".");
      // assert_eq!("foo", format!("{ans:?}"));  // shows sub-directories
      assert_eq!(4, ans.len());
   }
}
