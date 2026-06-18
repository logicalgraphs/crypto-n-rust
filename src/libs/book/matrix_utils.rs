use std::{ clone::Clone, fmt::Debug, str::FromStr };

use crate::{
   err_utils::ErrStr,
   list_utils::{filter_map_or,parse_nums},
   string_utils::s
};

pub type Matrix<T> = Vec<Vec<T>>;

pub fn from_vec<T>(v: Vec<T>) -> Matrix<T> {
    let mut mat = Vec::new();
    mat.push(v);
    mat
}

pub fn from_lines<T: FromStr>(lines: &[&str], separator: &str)
      -> ErrStr<Matrix<T>> where <T as FromStr>::Err: Debug {
   filter_map_or(|line: &str| {
      let eaches: Vec<String> = line.split(separator).map(s).collect();
      parse_nums::<T>(&eaches)
   }, lines.to_vec())
}

pub fn col<T: Clone>(rows: &Matrix<T>, col: usize) -> Vec<T> {
    fn column<T: Clone>(c: usize) -> impl Fn(&Vec<T>) -> T {
        move |row| row[c].clone()
    }
    rows.into_iter().map(column(col)).collect()
}

pub fn transpose<T: Clone>(cols: &Matrix<T>) -> Matrix<T> {
    let mut ans = Vec::new();
    if let Some(rows) = cols.first() {
        for ix in 0..rows.len() {
            let row = col(&cols, ix);
            ans.push(row);
        }
    }
    ans
}

pub fn print_matrix<T: Debug>(mat: &Matrix<T>) {
   mat.iter().for_each(|row| println!("{row:?}"));
}

// ----- TESTS -------------------------------------------------------

#[cfg(test)]
#[cfg(not(tarpaulin_include))]
mod sample_matrices {

   use super::*;

   pub fn sudoku() -> Matrix<i32> {
      vec![vec![1, 2, 3], vec![4, 5, 6], vec![7, 8, 9]]
   }

   pub fn skinny() -> Matrix<i32> {
      vec![vec![1, 2], vec![2, 5], vec![5, 6]]
   }
}

#[cfg(test)]
#[cfg(not(tarpaulin_include))]
pub mod functional_tests {
   use paste::paste;

   use super::*;
   use super::sample_matrices::{skinny,sudoku};
   use crate::{ create_testing, compose, utils::debug };

   create_testing!("matrix_utils");

   run!("matrix", " (from vec)", {
      let s = sudoku();
      print_matrix(&s);
   });

   run!("transpose", {
      let ski = skinny();
      print_matrix(&ski);
      let t = transpose(&ski);
      println!("\ntransposed:\n");
      print_matrix(&t);
   });

   run_with!("from_vec", vec![1,4,3], compose!(debug)(from_vec));
   run!("from_lines", {
      let src = vec!["1 2 3 4", "6 4 3 7", "9 2 6 7"];
      let mat = from_lines::<usize>(&src, " ")?;
      print_matrix(&mat);
   });
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
    use super::*;
    use super::sample_matrices::{skinny,sudoku};

    #[test] fn test_col() {
        let test_matrix = sudoku();
        let b = col(&test_matrix, 1);
        assert_eq!(vec![2, 5, 8], b);
    }

    #[test] fn test_transpose() {
        let test_matrix = skinny();
        let b = transpose(&test_matrix);
        assert_eq!(vec![vec![1, 2, 5], vec![2, 5, 6]], b);
    }
}

