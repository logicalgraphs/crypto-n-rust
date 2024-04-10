use std::clone::Clone;

use crate::string_utils::to_string;

pub type Matrix<T> = Vec<Vec<T>>;

pub fn from_lines(lines: &Vec<String>, separator: &str) -> Matrix<String> {
   lines.into_iter().map(|l| {
           let line = l.split(separator).map(to_string);
           line.collect()
        }).collect()
}

pub fn column_view<T: Clone>(rows: &Matrix<T>, col: usize) -> Vec<T> {
   fn column<T: Clone>(c: usize) -> impl Fn(&Vec<T>) -> T {
      move |row| row[c].clone()
   }
   rows.into_iter().map(column(col)).collect()
}
