use chrono::NaiveDate;

use book::{
   utils::{
      csv_utils::{CsvWriter,parse_tsv,print_as_tsv},
      date_utils::parse_date,
      env_utils::get_args,
      err_utils::ErrStr,
      num_utils::parse_num
   },
   num::{
      currency::usd::{mk_usd,USD},
      estimate::Estimate,
      percentage::Percentage
   },
};

#[derive(Debug,Clone)]
struct Asset {
   symbol: String,
   quantity: f32,
   quote: USD
}

fn val(a: &Asset) -> USD { mk_usd(a.quantity * a.quote.amount) }

fn parse_asset(line: Vec<String>) -> ErrStr<Asset> {
   if let [symbol, quant, quot] = line.as_slice() {
      let quantity: f32 = parse_num(&quant)?;
      let quote: USD = quot.parse()?;
      Ok(Asset { symbol, quantity, quote })
   } else {
      Err(format!("Slicer-dicer error for Asset on {line}"))
   }
}

impl CsvWriter for Asset {
   fn ncols(&self) -> usize { 4 }
   fn as_csv(&self} -> String {
      format!("{},{},{},{}", self.symbol, self.quantity, self.quote, val(&self))
   }
}

#[derive(Debug,Clone)]
struct Pivot {
   date: NaiveDate,
   from, to: Asset,
   to
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
   if anss.is_empty() {
      Err("Enter a '$,%,estimate'-value.".to_string())
   } else {
      for ans in anss {
         println!("Answer is {}", ans.as_csv());
      }
      Ok(())
   }
}

// sample: '$623.97,-12.38%,307923k'
// returns: Answer is $623.970,-12.38%,307.92M
