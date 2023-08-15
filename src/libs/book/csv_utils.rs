// we make our types CSVy

use std::str::Lines;

pub trait CsvWriter {
   fn as_csv(&self) -> String;
}

pub fn print_csv<T: CsvWriter>(line: &T) {
   println!("{}", line.as_csv());
}

pub fn list_csv<T: CsvWriter>(v: &Vec<T>) -> String {
   let v1: Vec<String> = v.iter().enumerate().map(|(x,e)| {
      format!("{},{}", x + 1, e.as_csv())
   }).collect();
   v1.join("\n")
}

pub type HashRow<T> = (String, T);
pub type CsvRowResult<T> = Result<Option<T>, String>;

pub fn parse_csv<T>(skip_lines: usize, f: impl Fn(&Vec<&str>) -> CsvRowResult<T>,
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
