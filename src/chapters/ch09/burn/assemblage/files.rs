// from "USK,wBTC,axlUSDC,wBTC,USK" returns a set of files or err

use std::{
   collections::HashSet,
   fs::read_dir
};

use book::string_utils::dequote;

pub fn files(trade: &Vec<&str>, files: &HashSet<String>)
   -> Result<HashSet<String>, String> {
   files_d(trade, files, false)
}

pub fn files_d(trade: &Vec<&str>, files: &HashSet<String>, debug: bool)
   -> Result<HashSet<String>, String> {
   let mut ans = HashSet::new();
   for window in trade.windows(2) {
      if debug { println!("Processing {window:?}"); }
      let file = file_in(&window[0], &window[1], &files)?;
      ans.insert(file);
   }
   Ok(ans)
}

pub fn collect_files(dir: &str) -> Result<HashSet<String>, String> {
   collect_files_d(dir, false)
}

pub fn collect_files_d(dir: &str, debug: bool)
   -> Result<HashSet<String>, String> {
   let entries = read_dir(dir).expect(&format!("could not read_dir {dir}"));
   let mut files = HashSet::new();
   for file in entries {
      if let Ok(file) = file {
         files.insert(dequote(format!("{:?}", file.file_name())));
      }
   }
   if debug { println!("Files in dir: {files:?}"); }
   Ok(files)
}

type FilePair = (String, String);

fn mk_filename(a: &str, b: &str) -> FilePair {
   (format!("{a}_{b}.json"), format!("{b}_{a}.json"))
}

fn file_in(x: &str, y: &str, files: &HashSet<String>)
   -> Result<String, String> {
   let (a, b) = mk_filename(x, y);
   if files.contains(&a) { Ok(a) } else {
   if files.contains(&b) { Ok(b) } else {
   Err(format!("{a} not a monitored order book")) } }
}
   
