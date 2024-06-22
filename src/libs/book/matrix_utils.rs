use std::clone::Clone;

pub type Matrix<T> = Vec<Vec<T>>;

pub fn from_split_line<T>(f: impl Fn(&str) -> T)
      -> impl Fn(Vec<&str>) -> Vec<T> {
   move |line| line.into_iter().map(&f).collect()
}

pub fn from_lines<T>(f: impl Fn(&str) -> T,
                     lines: &Vec<String>, separator: &str) -> Matrix<T> {
      lines.into_iter()
           .map(|l| from_split_line(&f)(l.split(separator).collect()))
           .collect()
}

pub fn col<T: Clone>(rows: &Matrix<T>, col: usize) -> Vec<T> {
   fn column<T: Clone>(c: usize) -> impl Fn(&Vec<T>) -> T {
      move |row| row[c].clone()
   }
   rows.into_iter().map(column(col)).collect()
}
