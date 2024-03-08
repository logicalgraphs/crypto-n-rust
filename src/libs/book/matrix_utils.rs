use std::clone::Clone;

pub type Matrix<T> = Vec<Vec<T>>;

pub fn column_view<T: Clone>(rows: &Matrix<T>, col: usize) -> Vec<T> {
   fn column<T: Clone>(c: usize) -> impl Fn(&Vec<T>) -> T {
      move |row| row[c].clone()
   }
   rows.into_iter().map(column(col)).collect()
}
