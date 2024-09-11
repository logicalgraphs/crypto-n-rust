// simple case: we take two tables and merge them, easy-peasies!

use std::collections::HashMap;
use book::{
   csv_utils::CsvWriter,
   table_utils::{Table,  /* from_map, */ merge}
};

fn map_table(row: &str, h: &HashMap<&str, f32>) -> Table<String, String, f32> {
   panic!("map_table not yet implemented")
}

fn main() {
   let a = HashMap::from([("Apple", 1.1), ("Banana", 2.3), ("Chocolate", 3.14)]);
   let b = HashMap::from([("Apple", 2.2), ("Chocolate", 9.7)]);
   let t1 = map_table("ETH", &a);
   let t2 = map_table("BTC", &b);
   let m = merge(&t1, &t2);
   println!("New table is
{}", m.as_csv());
}
