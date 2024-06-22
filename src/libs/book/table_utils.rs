use std::{
   cmp::Eq,
   collections::HashMap,
   hash::Hash
};

use crate::{
   compose,
   list_utils::{ht,tail},
   matrix_utils::{Matrix, /* column_view, */ from_split_line},
   string_utils::to_string,
   tuple_utils::swap
};

// a Table is a matrix indexed by hashed values, so we can have, e.g.:
// Table<NaiveDate, String, T> which maps to Matrix<T> with rows as dates
// and columns as strings.

// I accomplish by the mapping 2024-03-10 -> 0, 2024-03-11 -> 1, ... for rows
// and ADA -> 0, ATOM -> 1, AVAX -> 2, ... for columns.

// (the mappings can be any <T:Hash,Eq>-typed values)

pub struct Table<Row, Col, T> {
   rows: HashMap<Row, usize>,
   cols: HashMap<Col, usize>,
   data: Matrix<T>
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
