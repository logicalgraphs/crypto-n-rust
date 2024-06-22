use std::{
   clone::Clone,
   cmp::Eq,
   collections::{HashMap,HashSet},
   hash::Hash
};

use crate::{
   compose,
   list_utils::{ht,tail},
   matrix_utils,
   matrix_utils::{Matrix, /* col, */ from_split_line},
   string_utils::to_string,
   tuple_utils::{snd,swap}
};

// a Table is a matrix indexed by hashed values, so we can have, e.g.:
// Table<NaiveDate, String, T> which maps to Matrix<T> with rows as dates
// and columns as strings.

// I accomplish by the mapping 2024-03-10 -> 0, 2024-03-11 -> 1, ... for rows
// and ADA -> 0, ATOM -> 1, AVAX -> 2, ... for columns.

// (the mappings can be any <T:Hash,Eq>-typed values)

pub struct Table<Row, Col, T> {
   pub rows: HashMap<Row, usize>,
   pub cols: HashMap<Col, usize>,
   pub data: Matrix<T>
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
      let (rows, data) = rows_in_jest(rowf, df, &body, separator);
      Table { rows, cols: parse_headers(colf, &cols_str), data }
   } else {
      panic!("No table to ingest!")
   }
}

fn parse_headers<HEADER: Eq + Hash>(headerf: impl Fn(&str) -> HEADER,
                         headers: &Vec<String>) -> HashMap<HEADER, usize> {
   let hdrs: Vec<HEADER> =
      headers.into_iter().map(compose!(&headerf)(String::as_str)).collect();
   hdrs.into_iter().enumerate().map(swap).collect()
}

// rows_in_jest is a funny function ... GEDDIT? ;)

fn rows_in_jest<ROW: Eq + Hash, DATUM>(rowf: impl Fn(&str) -> ROW,
                            df:   impl Fn(&str) -> DATUM,
                            lines: &Vec<String>, separator: &str)
      -> (HashMap<ROW, usize>, Matrix<DATUM>) {
   let rows: Vec<Vec<&str>> =
      lines.into_iter().map(|l| l.split(separator)
                        //         .map(to_string)
                                 .collect()).collect();
   let (mbs_hdrs, data): (Vec<Option<&str>>, Vec<Vec<&str>>) =
      rows.iter().map(ht).unzip();
   let hdrs: Vec<String> =
      mbs_hdrs.into_iter()
              .map(compose!(to_string)(Option::unwrap))
              .collect();
   let matrix = data.into_iter().map(from_split_line(&df)).collect();
   (parse_headers(rowf, &hdrs), matrix)
}

pub fn col<ROW, COL: Eq + Hash, DATUM: Clone>(table: &Table<ROW, COL, DATUM>,
                                              cix: &COL) -> Option<Vec<DATUM>> {
   table.cols.get(cix).and_then(|c| Some(matrix_utils::col(&table.data, *c)))
}

pub fn row_filter<ROW: Clone, COL: Clone, DATA: Clone>(f: impl Fn(&ROW) -> bool,
                                   table: &Table<ROW, COL, DATA>)
      -> Table<ROW, COL, DATA> {
   // for the new table, 
   // 1. the columns are the columns, so that's no biggie
   let cols = table.cols.clone();

   // 2. we filter the rows
   let mut rows = table.rows.clone();
   rows.retain(|k,_v| f(k));
   
   // 3. now we filter the data by rows
   let row_ixen: HashSet<usize> = rows.values().map(|v| *v).collect();
   let data: Matrix<DATA> =
      table.data.clone()
                .into_iter()
                .enumerate()
                .filter(|(ix,_row)| row_ixen.contains(ix))
                .map(snd)
                .collect();
   Table { rows, cols, data }

   // now a call to col() on this new table will be properly filtered.
}
