use chrono::DateTime;
use serde_json::from_reader;

use std::{collections::HashMap,fmt::Debug,fs::File,io::BufReader,path::Path};

use book::{
   err_utils::{ErrStr,err_or},
   types::{Stamped,stamp}
};

type StampedPrice0 = Vec<f64>;
type StampedData<A> = Vec<A>;
type Chart<A> = HashMap<String, StampedData<A>>;

fn read_chart_from_file0<P: AsRef<Path> + Debug + Clone>(path: P)
        -> ErrStr<Chart<StampedPrice0>> {
    // Open the file in read-only mode with buffer.
    let p = path.clone();
    let file = err_or(File::open(p), &format!("Cannot open {:?}", path))?;
    let reader = BufReader::new(file);

    // Read the JSON contents of the file as an instance of the chart-data
    let chart = err_or(from_reader(reader), "Cannot parse JSON")?;

    Ok(chart)
}

fn read_chart_from_file<P: AsRef<Path> + Debug + Clone>(path: P)
        -> ErrStr<Chart<Stamped<f64>>> {
   let raw = read_chart_from_file0(path)?;
   let mut ans = HashMap::new();
   fn to_stamp(v: &Vec<f64>) -> Stamped<f64> {
      let dt = DateTime::from_timestamp((v[0] / 1000.0) as i64, 0).unwrap();
      stamp(&dt.date_naive(), &v[1])
   }
   for (k,v) in raw {
      ans.insert(k, v.iter().map(to_stamp).collect());
   }
   Ok(ans)
}

fn main() -> ErrStr<()> {
    let chart = read_chart_from_file("data/eth.json")?;
    for section in chart {
       print_section(&section);
    }
   Ok(())
}

fn print_section<A: Debug>((section, row): &(String, StampedData<A>)) {
   println!("Section: {section}");

   fn print_datum<A: Debug>(data: &A) {
      println!("\t{:?}", data);
   }
   row.iter().take(3).for_each(print_datum);
   println!("\t...");
}
