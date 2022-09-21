// we make our types CSVy

use crate::crypto_types::{Coin,csv,parse_coin};

pub fn read_csv_coin(line: &String) -> Result<Coin, String> {
   if let [date,cmc_id,rank,symbol,name,price] =
         line.split(',').collect::<Vec<&str>>().as_slice() {
      parse_coin(date, cmc_id, rank, symbol, name, price)
   } else {
      // panic!("Can't parse line: {}", line)
      Err("Can't parse line: ".to_owned() + line)
   }
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
