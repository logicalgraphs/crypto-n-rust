use serde_json::from_reader;

use std::{collections::HashMap,fmt::Debug,fs::File,io::BufReader,path::Path};

use book::err_utils::{ErrStr,err_or};

type StampedData = Vec<Vec<f64>>;
type Chart = HashMap<String, StampedData>;

fn read_chart_from_file<P: AsRef<Path> + Debug + Clone>(path: P)
        -> ErrStr<Chart> {
    // Open the file in read-only mode with buffer.
    let p = path.clone();
    let file = err_or(File::open(p), &format!("Cannot open {:?}", path))?;
    let reader = BufReader::new(file);

    // Read the JSON contents of the file as an instance of the chart-data
    let chart = err_or(from_reader(reader), "Cannot parse JSON")?;

    Ok(chart)
}

fn main() -> ErrStr<()> {
    let chart = read_chart_from_file("data/eth.json")?;
    for section in chart {
       print_section(&section);
    }
   Ok(())
}

fn print_section((section, row): &(String, Vec<Vec<f64>>)) {
   println!("Section: {section}");

   fn print_datum(data: &Vec<f64>) {
      println!("\t{:?}", data);
   }
   row.into_iter().take(3).for_each(print_datum);
   println!("\t...");
}
