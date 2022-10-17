// we make our types CSVy

pub trait CsvWriter {
   fn as_csv(&self) -> String;
}

pub fn print_csv<T: CsvWriter>(line: &T) {
   println!("{}", line.as_csv());
}
