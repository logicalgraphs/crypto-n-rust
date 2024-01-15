// we make our types CSVy

use std::str::Lines;

use chrono::naive::NaiveDate; 

use crate::list_utils::mk_cycle;

// ----- Types -------------------------------------------------------

pub trait CsvWriter {
   fn as_csv(&self) -> String;
   fn ncols(&self) -> usize;
}

pub struct Stamped<T> {
   pub date: NaiveDate,
   pub pack: T
}

pub fn stamp<T: Clone>(date: &NaiveDate, pack: &T) -> Stamped<T> {
   Stamped { date: date.clone(), pack: pack.clone() }
}

impl<T:CsvWriter> CsvWriter for Stamped<T> {
   fn as_csv(&self) -> String {
      format!("{},{}", self.date, self.pack.as_csv())
   }
   fn ncols(&self) -> usize { 1 + self.pack.ncols() }
}

pub struct Indexed<T> {
   pub idx: usize,
   pub pack: T
}

impl<T:CsvWriter> CsvWriter for Indexed<T> {
   fn as_csv(&self) -> String {
      format!("{},{}", self.idx, self.pack.as_csv())
   }
   fn ncols(&self) -> usize { 1 + self.pack.ncols() }
}

// useful when enumerating over a Vec: map this fn to make an Indexed-value

pub fn mk_idx<T: Clone>(i: usize, p: &T) -> Indexed<T> {
   Indexed { idx: i, pack: p.clone() }
}

pub fn mk_idx_offset<T: Clone>(pear: (usize, &T)) -> Indexed<T> {
   let (i, p) = pear;
   Indexed { idx: i+1, pack: p.clone() }
}

// ----- Printers -------------------------------------------------------

pub fn print_csv<T: CsvWriter>(line: &T) {
   println!("{}", line.as_csv());
}

pub fn print_as_tsv(row: &String) {
   let cols: Vec<&str> = row.split(",").collect();
   println!("{}", cols.join("\t"));
}

pub fn list_csv<T: CsvWriter>(v: &Vec<T>) -> String {
   let v1: Vec<String> = v.iter().enumerate().map(|(x,e)| {
      format!("{},{}", x + 1, e.as_csv())
   }).collect();
   v1.join("\n")
}

// ----- Parsers -------------------------------------------------------

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

// ----- Formatters -------------------------------------------------------

// puts CSV side-by-side in columns with 1 skip-column between each type

pub fn columns(csvs: &Vec<Vec<ToCsv>>) -> Vec<String> {
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
   rows
}

// ... and helper functions for columns

pub struct ToCsv {  // we flatten our structure, T, into its CSV-representation
   row: String,
   ncols: usize
}

pub fn mk_csvs<T: CsvWriter>(rows: &Vec<T>) -> Vec<ToCsv> {
   rows.iter().map(mk_csv).collect()
}

fn mk_csv<T: CsvWriter>(row: &T) -> ToCsv {
   ToCsv { row: row.as_csv(), ncols: row.ncols() }
}

struct Blank {      // prints a blank row
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

fn as_csv_or_blank_at(i: usize) -> impl Fn(&Vec<ToCsv>) -> String {
   move | vec | {
      if let Some(row) = vec.get(i) {
         row.row.clone()
      } else {
         if let Some(sample) = vec.first() {
            mk_blank(sample.ncols).as_csv()
         } else {
            panic!("Column is empty!")
         }
      }
   }
}
