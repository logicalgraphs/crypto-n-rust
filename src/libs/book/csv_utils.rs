// we make our types CSVy

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
