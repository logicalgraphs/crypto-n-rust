// we make our types CSVy

use crate::{
   err_utils::ErrStr,
   list_utils::mk_cycle,
   string_utils::{parse_lines,s}
};

// ----- Types -------------------------------------------------------

pub trait CsvHeader {
   fn header(&self) -> String;
}

pub trait CsvWriter {
   fn as_csv(&self) -> String;
   fn ncols(&self) -> usize;
}

impl CsvWriter for i32 {
   fn as_csv(&self) -> String { format!("{}", &self) }
   fn ncols(&self) -> usize { 1 }
}

// ----- Printers -------------------------------------------------------

pub fn print_csv<T: CsvWriter>(line: &T) {
   print_line(&line.as_csv());
}

pub fn print_line(line: &String) {
   println!("{line}");
}

pub fn print_as_tsv(row: &String) {
   let cols: Vec<&str> = row.split(",").collect();
   print_line(&cols.join("\t"));
}

pub fn list_csv<T: CsvWriter>(v: &Vec<T>) -> String {
   let v1: Vec<String> = v.iter().map(|e| {
      format!("{}", e.as_csv())
   }).collect();
   v1.join("\n")
}

pub fn enumerate_csv<T: CsvWriter>(v: &Vec<T>) -> String {
   let v1: Vec<String> = v.iter().enumerate().map(|(x,e)| {
      format!("{},{}", x + 1, e.as_csv())
   }).collect();
   v1.join("\n")
}

// ----- Parsers -------------------------------------------------------

type ParserFn<T> = dyn Fn(Vec<String>) -> ErrStr<T>;

fn parser<T>(separator: &str, skip_lines: usize, 
             f: &ParserFn<T>, lines: &Vec<String>) -> ErrStr<Vec<T>> {
   fn cols(sep: &str) -> impl Fn(String) -> Vec<String> + '_ {
      move |line| line.split(sep).map(s).collect()
   }
   fn g<'a, T>(s: &'a str, f: &'a ParserFn<T>)
         -> impl Fn(String) -> ErrStr<T> + 'a {
      move |line| f(cols(s)(line))
   }
   parse_lines(g(separator, f), lines, Some(skip_lines))
}

// Got CSV? This fn will parse that CSV into Vec<T>

pub fn parse_csv<T>(skip_lines: usize, f: &ParserFn<T>, lines: &Vec<String>)
      -> ErrStr<Vec<T>> {
   parser(",", skip_lines, f, lines)
}

pub fn parse_tsv<T>(skip_lines: usize, f: &ParserFn<T>, lines: &Vec<String>)
      -> ErrStr<Vec<T>> {
   parser("\t", skip_lines, f, lines)
}

// ----- Formatters -------------------------------------------------------

// puts CSV side-by-side in columns with optional skip-column between each type

pub fn columns(csvs: &Vec<Vec<ToCsv>>, sep: usize) -> Vec<String> {
   let separator =
      if sep > 0 { format!(",{}", mk_blank(sep).as_csv()) } else { s("") };
   let mut max = 0;
   for r in csvs { if r.len() > max { max = r.len(); } }
   let mut rows: Vec<String> = Vec::new();
   for i in 0..max {
      let row: Vec<String> =
          csvs.into_iter().map(as_csv_or_blank_at(i)).collect();
      rows.insert(i, row.join(&separator));
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

pub struct Blank {      // prints a blank row
   s: Vec<String>,
   n: usize
}

pub fn mk_blank(n: usize) -> Blank {
   let eh = mk_cycle(&s(" "));  // odd syntax to borrow Clone-trait
   let s: Vec<String> = eh.iter().take(n).collect();
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
