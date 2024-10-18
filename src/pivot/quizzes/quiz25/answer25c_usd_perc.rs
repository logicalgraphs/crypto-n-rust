use book::{
   csv_utils::{CsvWriter,parse_csv},
   err_utils::ErrStr,
   num::{
      estimate::Estimate,
      percentage::Percentage,
      usd::USD
   },
   utils::get_args
};

#[derive(Debug,Clone)]
struct USDPerc {
   u: USD,
   p: Percentage,
   e: Estimate
}

impl CsvWriter for USDPerc {
   fn ncols(&self) -> usize { 3 }
   fn as_csv(&self) -> String {
      format!("{},{},{}", self.u, self.p, self.e)
   }
}

fn parser(line: Vec<String>) -> ErrStr<USDPerc> {
   if let [usd, perc, est] = line.as_slice() {
      let u: USD = usd.parse()?;
      let p: Percentage = perc.parse()?;
      let e: Estimate = est.parse()?;
      Ok(USDPerc { u, p, e })
   } else {
      Err(format!("USDPerc parse failed on '{line:?}'"))
   }
}

fn main() -> ErrStr<()> {
   let args = get_args();
   let anss = parse_csv(0, &parser, &args)?;
   for ans in anss {
      println!("Answer is {}", ans.as_csv());
   }
   Ok(())
}

// sample: '$623.97,-12.38%,307923k'
// returns: Answer is $623.970,-12.38%,307.92M
