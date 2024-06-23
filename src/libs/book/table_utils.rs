use std::{
   clone::Clone,
   cmp::Eq,
   collections::{HashMap,HashSet},
   fmt::Display,
   hash::Hash
};

use crate::{
   compose,
   csv_utils::CsvWriter,
   list_utils::{ht,tail},
   matrix_utils,
   matrix_utils::{Matrix, from_split_line},
   string_utils::to_string,
   tuple_utils::{fst,snd,swap},
   utils::pred
};

// a Table is a matrix indexed by hashed values, so we can have, e.g.:
// Table<NaiveDate, String, T> which maps to Matrix<T> with rows as dates
// and columns as strings.

// I accomplish by the mapping 2024-03-10 -> 0, 2024-03-11 -> 1, ... for rows
// and ADA -> 0, ATOM -> 1, AVAX -> 2, ... for columns.

// (the mappings can be any <T:Hash,Eq>-typed values)

pub struct Table<ROW, COL, T> {
   rows_: HashMap<ROW, usize>,
   cols_: HashMap<COL, usize>,
   pub data: Matrix<T>
}

fn enum_headers<HEADER: Eq + Hash>(headers: Vec<HEADER>)
      -> HashMap<HEADER, usize> {
   headers.into_iter().enumerate().map(swap).collect()
}

fn sort_headers<HEADER: Clone>(hdrs: &HashMap<HEADER,usize>) -> Vec<HEADER> {
   let mut hdrs1: Vec<(&HEADER,&usize)> = hdrs.into_iter().collect();
   hdrs1.sort_by(|a,b| a.1.cmp(&b.1));
   hdrs1.into_iter().map(compose!(Clone::clone)(fst)).collect()
}

pub fn rows<ROW: Clone,COL,T>(table: &Table<ROW,COL,T>) -> Vec<ROW> {
   sort_headers(&table.rows_)
}

pub fn cols<ROW,COL: Clone,T>(table: &Table<ROW,COL,T>) -> Vec<COL> {
   sort_headers(&table.cols_)
}

pub fn mk_table<ROW: Eq + Hash, COL: Eq + Hash, T>(r: Vec<ROW>, c: Vec<COL>,
      data: Matrix<T>) -> Table<ROW, COL, T> {
   let rows = enum_headers(r);
   let cols = enum_headers(c);
   Table { rows_: rows, cols_: cols, data }
}

impl <ROW: Display + Clone,COL: Display + Clone,DATUM: Display> CsvWriter
      for Table<ROW,COL,DATUM> {
   fn ncols(&self) -> usize { self.cols_.len() + 1 }
   fn as_csv(&self) -> String {
      let cols = display_cols(&self.cols_);
      let rows = sort_headers(&self.rows_);
      fn fmt_row<ROW: Display, T: Display>((header, row): (&ROW, &Vec<T>))
            -> String {
         format!("{},{}", header, vec_to_vec(&row).join(","))
      }
      let data: Vec<String> = rows.iter()
                                  .zip(self.data.iter())
                                  .map(fmt_row)
                                  .collect();
      format!("{}\n{}", cols, data.join("\n"))
   }
}

fn vec_to_vec<T: Display>(line: &Vec<T>) -> Vec<String> {
   line.into_iter().map(|s| format!("{}", s)).collect()
}

fn display_cols<COL: Display + Clone>(cols: &HashMap<COL,usize>) -> String {
   format!(",{}", vec_to_vec(&sort_headers(cols)).join(","))
}

pub fn ingest<ROW: Eq + Hash,COL: Eq + Hash,DATUM>(rowf: impl Fn(&str) -> ROW,
                             colf: impl Fn(&str) -> COL,
                             df:   impl Fn(&str) -> DATUM,
                             lines: &Vec<String>, separator: &str)
      -> Table<ROW,COL,DATUM> {

   // the first row is the column-headers:
   let (header, body) = ht(lines);
   if let Some(hdr) = header {
      let cols_str: Vec<String> =
         tail(&hdr.split(separator).map(to_string).collect());
      let (rows, data) = rows_in_jest(rowf, df, body, separator);
      Table { rows_: rows, cols_: parse_headers(colf, &cols_str), data }
   } else {
      panic!("No table to ingest!")
   }
}

fn parse_headers<HEADER: Eq + Hash>(headerf: impl Fn(&str) -> HEADER,
                         headers: &Vec<String>) -> HashMap<HEADER, usize> {
   let hdrs: Vec<HEADER> =
      headers.into_iter().map(compose!(&headerf)(String::as_str)).collect();
   enum_headers(hdrs)
}

// rows_in_jest is a funny function ... GEDDIT? ;)

fn rows_in_jest<ROW: Eq + Hash, DATUM>(rowf: impl Fn(&str) -> ROW,
                            df:   impl Fn(&str) -> DATUM,
                            lines: Vec<String>, separator: &str)
      -> (HashMap<ROW, usize>, Matrix<DATUM>) {
   fn split_line_p(separator: &str)
         -> impl Fn(String) -> Option<Vec<String>> + '_ {
      move |s| pred(!s.is_empty(), s.split(separator).map(to_string).collect())
   }
   let rows: Vec<Vec<String>> =
      lines.into_iter()
           .filter_map(&split_line_p(separator))
           .collect();
   let (mbs_hdrs, data): (Vec<Option<String>>, Vec<Vec<String>>) =
      rows.iter().map(ht).unzip();
   let hdrs: Vec<String> = mbs_hdrs.into_iter().map(Option::unwrap).collect();
   fn deref<'a>(v: &'a Vec<String>) -> Vec<&'a str> {
      v.iter().map(String::as_str).collect()
   }
   let matrix = data.iter()
                    .map(compose!(from_split_line(&df))(deref))
                    .collect();
   (parse_headers(rowf, &hdrs), matrix)
}

pub fn col<ROW, COL: Eq + Hash, DATUM: Clone>(table: &Table<ROW, COL, DATUM>,
                                              cix: &COL) -> Option<Vec<DATUM>> {
   table.cols_.get(cix).and_then(|c| Some(matrix_utils::col(&table.data, *c)))
}

pub fn row_filter<ROW: Clone + Eq + Hash, COL: Clone + Eq + Hash, DATA: Clone>
      (f: impl Fn(&ROW) -> bool, table: &Table<ROW, COL, DATA>)
      -> Table<ROW, COL, DATA> {
   // for the new table, 
   // 1. the columns are the columns, so that's no biggie
   let cols = sort_headers(&table.cols_);

   // 2. we filter the rows
   let mut rs = table.rows_.clone();
   rs.retain(|k,_v| f(k));

   // 3. now we filter the data by rows
   let row_ixen: HashSet<usize> = rs.values().map(|v| *v).collect();
   let rows = sort_headers(&rs);
   let data: Matrix<DATA> =
      table.data.clone()
                .into_iter()
                .enumerate()
                .filter(|(ix,_row)| row_ixen.contains(ix))
                .map(snd)
                .collect();

   mk_table(rows, cols, data)

   // now a call to col() on this new table will be properly filtered.
}
