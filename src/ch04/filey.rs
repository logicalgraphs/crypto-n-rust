// from https://stackoverflow.com/questions/30801031/read-a-file-and-get-an-array-of-strings

use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

mod utils;

fn main() {
   let files = utils::get_args();

   for file in files {
      let lines = lines_from_file(&file); 
      // Consumes the iterator, returns an (Optional) String
      println!("File {}:\n", file);
      for line in &lines {
         println!("{}", line);
      }
      println!("\nThe fifh line is:\n{}\n", lines[4]);
   }
}

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}
