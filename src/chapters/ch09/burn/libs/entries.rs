use book::csv_utils::CsvWriter;

#[derive(Debug, Clone)]
pub struct Entry {
   pub ratio: f32,
   pub amount: f32
}

// ----- Printing functions -----------------------------------------

impl CsvWriter for Entry {
   fn as_csv(&self) -> String { format!("{},{}", self.ratio, self.amount) }
   fn ncols(&self) -> usize { 2 }
}
