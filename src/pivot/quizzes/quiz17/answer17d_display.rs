use book::{
   csv_utils::print_csv,
   err_utils::ErrStr,
   table_utils::{from_map, transpose}
};

use swerve::{
   fetch_prices::read_chart_from_file,
   types::StampedData
};

fn main() -> ErrStr<()> {
   let chart = read_chart_from_file("data/eth.json")?;
   let stamped_prices: &StampedData<f64> = chart.get("prices").expect("price");
   let table = from_map("ETH", stamped_prices);
   let transposed = transpose(&table);
   print_csv(&transposed);
   Ok(())
}
