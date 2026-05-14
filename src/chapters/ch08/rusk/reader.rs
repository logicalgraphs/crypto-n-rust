// reads in the csv (converting kinda numbers to numbers)

use book::{
   csv_utils::{CsvWriter,print_csv},
   file_utils::extract_date_and_body,
   list_utils::tail,
   num_utils::parse_estimate,
   utils::get_args
};

struct Volume {
   n: u8,
   a: String,
   b: String,
   volume: f32
}

impl CsvWriter for Volume {
   fn as_csv(&self) -> String {
      format!("{},{},{},{}", self.n, self.a, self.b, self.volume)
   }
}

fn usage() {
   println!("./reader <file>");
   println!("\treads <file> and extracts order books and their volumes");
}

fn main() {
   if let Some(file) = get_args().first() {
      let (dt, lines) = extract_date_and_body(file);
      let header = "book #,lp-pair-1,lp-pair-2,volume";
      println!("date scraped:,{dt}\n\n{header}");
      for line in tail(lines) {
         let v: Volume =
            read_volume(&line)
               .expect(&format!("Could not read volume from {line}"));
         print_csv(&v);
      }
      // lines.iter().filter_map(read_volume).for_each(print_csv);
   } else {
      usage();
   }
}

fn read_volume(line: &str) -> Result<Volume, String> {
   let daters: Vec<&str> = line.split(',').collect();
   if let [n1,a1,b1,v1] = daters.as_slice() {
      let n: u8 =
         n1.parse().expect(&format!("Couldn't convert {n1} to index!"));
      let volume = parse_estimate(v1)?;
      let a = a1.to_string();
      let b = b1.to_string();
      Ok(Volume { n, a, b, volume })
   } else {
      Err(format!("Could not parse line: {line}"))
   }
}
