use chrono::NaiveDate;

use book::{
   csv_utils::print_csv,
   err_utils::ErrStr,
   table_utils::from_vec
};

use swerve::{
   fetch_prices::read_chart_from_file,
   types::StampedData
};

fn main() -> ErrStr<()> {
   let chart = read_chart_from_file("data/eth.json")?;
   let stamped_prices: &StampedData<f64> = chart.get("prices").expect("price");
   let dates: Vec<NaiveDate> =
      stamped_prices.keys().into_iter().map(NaiveDate::clone).collect();
   let prices: Vec<f64> =
      stamped_prices.values().into_iter().map(f64::clone).collect();
   let table = from_vec("ETH", dates, prices);
   print_csv(&table);
   Ok(())
}
