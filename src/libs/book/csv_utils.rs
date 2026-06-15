// we make our types CSVy

use std::fmt::Debug;
use serde::Serialize;

use crate::{
   err_utils::{err_or,ErrStr},
   list_utils::mk_cycle,
   string_utils::{parse_lines,s},
   utils::k
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

pub fn print_csv<T: CsvWriter>(line: &T) { print_line(&line.as_csv()); }

pub fn print_line(line: &String) { println!("{line}"); }

pub fn print_as_tsv(row: &String) {
   let cols: Vec<&str> = row.split(",").collect();
   print_line(&cols.join("\t"));
}

pub fn list_csv<T: CsvWriter>(v: &[T]) -> String {
   v.iter().map(|e| format!("{}", e.as_csv())).collect::<Vec<_>>().join("\n")
}

pub fn enumerate_csv<T: CsvWriter>(v: &Vec<T>) -> String {
   v.iter()
    .enumerate()
    .map(|(x,e)| format!("{},{}", x + 1, e.as_csv()))
    .collect::<Vec<_>>()
    .join("\n")
}

// ----- Serializer -------------------------------------------------------

fn as_str<T:Debug + Serialize>(sep: u8, v: &[T]) -> ErrStr<String> {
   let mut output = Vec::new();
   let mut wtr = csv::WriterBuilder::new().delimiter(sep)
                                          .from_writer(&mut output);
   for new_row in v {
      err_or(wtr.serialize(&new_row),
             &format!("Could not serialize row:\n{new_row:?}"))?;
   }
   let _ = err_or(wtr.flush(), "Unable to flush output to stdout");

   // Drop the writer to release its mutable borrow and guarantee all
   // bytes flush down to output_buffer
   std::mem::drop(wtr);

   // Convert the raw bytes buffer into a valid UTF-8 String
   err_or(String::from_utf8(output),
          &format!("Could not convert table to string"))
}

pub fn as_tsv<T:Debug + Serialize>(v: &[T]) -> ErrStr<String> {
   as_str(b'\t', v)
}

pub fn as_csv<T:Debug + Serialize>(v: &[T]) -> ErrStr<String> {
   as_str(b',', v)
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

pub fn columns(csvs: &[Vec<ToCsv>], sep: usize) -> Vec<String> {
   let separator =
      if sep > 0 { format!(",{}", mk_blank(sep).as_csv()) } else { s("") };
   let mut max_cols = 0;
   for r in csvs { let n = r.len(); if n > max_cols { max_cols = n; } }
   let mut rows: Vec<String> = Vec::new();
   for i in 0..max_cols {
      let row: Vec<String> =
          csvs.iter().map(as_csv_or_blank_at(i)).collect();
      rows.insert(i, row.join(&separator));
   }
   rows
}

// ... and helper functions for columns

// The point of ToCsv is to allow columns of disparate types of CsvWriter-types

pub struct ToCsv {  // we flatten our structure, T, into its CSV-representation
   row: String,
   ncols: usize
}

pub fn mk_csvs<T: CsvWriter>(rows: &[T]) -> Vec<ToCsv> {
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
impl CsvHeader for Blank {
   fn header(&self) -> String {
      self.s.iter().map(k("|")).collect::<Vec<_>>().join(",")
   }
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

// ----- TESTS -------------------------------------------------------

#[cfg(test)]
#[cfg(not(tarpaulin_include))]
mod test_data {
   use super::*;
   use serde::{Deserialize,de::DeserializeOwned};
   use serde_with::{serde_as, DisplayFromStr};
   use crate::{
      currency::usd::USD,
      parse_utils::{parse_id,parse_usd},
      string_utils::s
   };

   #[serde_as]
   #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
   pub struct Grocery {
      item: String,
      quantity: usize,
      #[serde_as(as = "DisplayFromStr")]
      price: USD
   }
   #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
   pub struct Store { id: usize, location: String }

   fn hdr_s() -> String { s("id,location") }
   impl CsvHeader for Store { fn header(&self) -> String { hdr_s() } }
   impl CsvWriter for Store {
      fn ncols(&self) -> usize { 2 }
      fn as_csv(&self) -> String { format!("{},{}", self.id, self.location) }
   }
   pub fn parse_store(line: Vec<String>) -> ErrStr<Store> {
      if let [i,loc] = line.as_slice() {
         Ok(Store { id: parse_id(i)?, location: s(loc) })
      } else {
         Err(format!("Cannot parse store from line: {line:?}"))
      }
   }

   fn hdr_g() -> String { s("item,quantity,price") }
   impl CsvHeader for Grocery { fn header(&self) -> String { hdr_g() } }
   impl CsvWriter for Grocery {
      fn ncols(&self) -> usize { 3 }
      fn as_csv(&self) -> String {
         format!("{},{},{}", self.item, self.quantity, self.price)
      }
   }

   pub fn parse_grocery(line: Vec<String>) -> ErrStr<Grocery> {
      if let [it,amt,quot] = line.as_slice() {
         Ok(Grocery { item: s(it),
                      quantity: parse_id(&amt)?,
                      price: parse_usd(&quot)? })
      } else {
         Err(format!("Cannot parse Grocery from line: {line:?}"))
      }
   }

   pub fn inventory() -> String { format!("{}
apples,15,$4.95
oranges,7,$2.23
bananas,8,$9.97
crisps,23,$8.86
beer,97,$23.55
", hdr_g())
   }

   pub fn stores() -> String { format!("{}
1,Annandale
2,Springfield
3,Alexandria
4,Arlington
", hdr_s())
   }

   // deserialization is in another library and not tested here
   pub fn items<T:DeserializeOwned>(s: &str) -> ErrStr<Vec<T>> {
      let mut r = csv::ReaderBuilder::new()
                     .delimiter(b',').from_reader(s.as_bytes());
      let mut ans = Vec::new();
      for item in r.deserialize() { ans.push(err_or(item, "Cannot parse")?); }
      Ok(ans)
   }

   pub fn lines(r: &str) -> Vec<String> { r.split("\n").map(s).collect() }
}

#[cfg(test)]
#[cfg(not(tarpaulin_include))]
mod functional_tests {
   use super::*;
   use super::test_data::{
      Grocery,
      Store,
      inventory,
      items,
      lines,
      parse_grocery,
      stores
   };
   use paste::paste;
   use crate::{create_testing,list_utils::init};

   create_testing!("csv_utils");

   run!("as_csv", " (Serialize)", {
      let groceries = items::<Grocery>(&inventory())?;
      println!("The groceries are:\n\n{}", as_csv(&groceries)?);
   });

   run!("as_tsv", " (Serialize)", {
      let groceries = items::<Grocery>(&inventory())?;
      println!("The groceries are:\n\n{}", as_tsv(&groceries)?);
   });

   run!("parse_csv", {
      let items = parse_csv(1, &parse_grocery, &init(&lines(&inventory())))?;
      let lines: Vec<String> = items.iter().map(CsvWriter::as_csv).collect();
      println!("Parse items from CSV:\n\n{}", lines.join("\n"));
   });

   run!("columns", {
      let groc = items::<Grocery>(&inventory())?;
      let stor = items::<Store>(&stores())?;
      let col1 = mk_csvs(&groc);
      let col2 = mk_csvs(&stor);
      let cols = columns(&[col1, col2], 1);
      println!("CSV tables in columns:\n\n{}", cols.join("\n"));
   });
}

#[cfg(test)]
#[cfg(not(tarpaulin_include))]
mod tests {
   use super::*;
   use super::test_data::{Grocery,inventory,items,lines,parse_grocery};
   use crate::list_utils::init;

   #[test] fn fail_parse_csv_nl_at_end() {
      let parsed_lines = parse_csv(1, &parse_grocery, &lines(&inventory()));
      assert!(parsed_lines.is_err());
   }

   #[test] fn parse_csv_ok() {
      let parsed_lines =
         parse_csv(1, &parse_grocery, &init(&lines(&inventory())));
      assert!(parsed_lines.is_ok());
   }

   #[test] fn parse_csv_and_deserialize_idempotent() -> ErrStr<()> {
      let parsed_lines =
         parse_csv(1, &parse_grocery, &init(&lines(&inventory())))?;
      let groceries = items::<Grocery>(&inventory())?;
      assert_eq!(groceries.len(), parsed_lines.len(), "length unequal");
      assert_eq!(groceries, parsed_lines);
      Ok(())
   }
}
