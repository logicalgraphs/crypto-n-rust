// we make our types CSVy

use crate::crypto_types::{Coin,csv};

trait CsvReader {
   fn read(line: String) -> Result<Self, String> where Self: Sized;
}

trait CsvWriter {
   fn as_csv(&self) -> String;
}

impl CsvWriter for Coin {
   fn as_csv(&self) -> String { csv(self) }
}

pub fn print_all_coins(coins: Vec<Coin>) {
   println!("There are {} coins.\n", coins.len());
   print_header();
   coins.iter().for_each(print_coin);
}

pub fn print_header() {
   println!("date,cmc_id,rank,symbol,name,price");
}

pub fn print_coin(coin: &Coin) {
   println!("{}", coin.as_csv());
}
