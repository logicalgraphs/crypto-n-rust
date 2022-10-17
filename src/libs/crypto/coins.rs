// types, regardless underlying data sources

use std::cmp::Ordering;

use crate::types::{USD,mk_usd};
use book::csv_utils::{CsvWriter,print_csv};

// ----- COIN -------------------------------------------------------

#[derive(Eq, Debug, Clone)]
pub struct Coin {
   date: String,
   cmc_id: u32,
   rank: u32,
   name: String,
   pub symbol: String,
   price: USD
}

impl CsvWriter for Coin {
   fn as_csv(&self) -> String { csv(self) }
}

impl Ord for Coin {
   fn cmp(&self, other: &Self) -> Ordering {
      self.symbol.cmp(&other.symbol)
   }
}

impl PartialOrd for Coin {
   fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
      Some(self.cmp(other))
   }
}

impl PartialEq for Coin {
   fn eq(&self, other: &Self) -> bool {
      self.symbol == other.symbol
   }
}

pub fn mk_coin(date: String, cmc_id: u32, rank: u32, name: String,
               symbol: String, amount: f32) -> Coin {
   Coin { date, cmc_id, rank, name, symbol, price: mk_usd(amount) }
}

fn parse_coin(dat: &str, id: &str, rnk: &str, sym: &str,
                  nam: &str, pric: &str) -> Result<Coin, String> {
   let price: USD = pric.parse().expect("dollahz");
   let cmc_id: u32 = id.parse().expect("id");
   let rank: u32 = rnk.parse().expect("rank");
   let date = dat.to_string();
   let name = nam.to_string();
   let symbol = sym.to_string();
   Ok(Coin { date, cmc_id, rank, name, symbol, price })
}

pub fn csv(coin: &Coin) -> String {
   format_args!("{},{},{},{},{},{}",
      coin.date, coin.cmc_id, coin.rank, coin.symbol, coin.name, coin.price)
      .to_string()
}

pub fn print_all_coins<T: CsvWriter>(coins: Vec<T>) {
   println!("There are {} coins.\n", coins.len());
   println!("date,cmc_id,rank,symbol,name,price");
   coins.iter().for_each(print_csv);
}

pub fn read_csv_coin(line: &String) -> Result<Coin, String> {
   if let [date,cmc_id,rank,symbol,name,price] =
         line.split(',').collect::<Vec<&str>>().as_slice() {
      parse_coin(date, cmc_id, rank, symbol, name, price)
   } else {
      Err("Can't parse line: ".to_owned() + line)
   }
}
