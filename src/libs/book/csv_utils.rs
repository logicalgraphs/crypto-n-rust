// we make our types CSVy

use std::str::Lines;

use crate::list_utils::mk_cycle;

pub trait CsvWriter {
   fn as_csv(&self) -> String;
   fn ncols(&self) -> usize;
}

pub fn print_csv<T: CsvWriter>(line: &T) {
   println!("{}", line.as_csv());
}

pub fn print_tsv<T: CsvWriter>(line: &T) {
   let row = line.as_csv();
   let cols: Vec<&str> = row.split(",").collect();
   println!("{}", cols.join(","));
}

pub fn list_csv<T: CsvWriter>(v: &Vec<T>) -> String {
   let v1: Vec<String> = v.iter().enumerate().map(|(x,e)| {
      format!("{},{}", x + 1, e.as_csv())
   }).collect();
   v1.join("\n")
}

// fn cols<T: CsvWriter>(c: &T) -> usize { c.ncols() }

// Got CSV? This fn will parse that CSV into Vec<T>

pub fn parse_csv<T>(skip_lines: usize,
                    f: impl Fn(&Vec<&str>) -> Result<Option<T>, String>,
                    lines: &mut Lines) -> Result<Vec<T>, String> {
   let mut ans: Vec<T> = Vec::new();
   let mut lines1 = lines.skip(skip_lines).peekable();
   while lines1.peek().is_some() {
      if let Some(line) = lines1.next() {
         let row: Vec<&str> = line.split(",").collect();
         let res = f(&row)?;
         if let Some(tea) = res {
            ans.push(tea);
         }
      } else {
         panic!("No next() when is_some() is true!");
      }
   }
   Ok(ans)
}

// puts CSV side-by-side in columns with 1 skip-column between each type

pub fn columns<T:CsvWriter + Clone>(csvs: &Vec<Vec<T>>) -> String {
   let mut max = 0;
   for r in csvs.iter() {
      if r.len() > max { max = r.len(); }
   }
   let mut rows: Vec<String> = Vec::new();
   for i in 0..max {
      let row: Vec<String> = csvs.into_iter()
                                 .map(as_csv_or_blank_at(i))
                                 .collect();
      rows.insert(i, row.join(", ,"));
   }
   rows.join("\n")
}

// ... and helper functions for columns

struct Blank {
   s: Vec<String>,
   n: usize
}

fn mk_blank(n: usize) -> Blank {
   let eh = mk_cycle(&" ".to_string());  // odd syntax to borrow Clone-trait
   let s: Vec<String> = eh.iter().take(n-1).collect();
   Blank { s, n }
}

impl CsvWriter for Blank {
   fn as_csv(&self) -> String {
      format!("{},", self.s.join(","))
   }
   fn ncols(&self) -> usize { self.n }
}

fn as_csv_or_blank_at<T: CsvWriter>(i: usize)
   -> impl Fn(&Vec<T>) -> String {
   move | vec | {
      if let Some(row) = vec.get(i) {
         row.as_csv()
      } else {
         if let Some(sample) = vec.first() {
            mk_blank(sample.ncols()).as_csv()
         } else {
            panic!("Column is empty!")
         }
      }
   }
}
