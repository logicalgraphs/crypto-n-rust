// simple case: we take two tables and merge them, easy-peasies!

use std::{collections::HashMap,fmt::Display};
use book::{
   csv_utils::CsvWriter,
   table_utils::{Table,from_map,merge}
};

fn print_table<A: Display + Clone,B: Display + Clone,C: Display>(name: &str,
     t: &Table<A,B,C>) {
   println!("{name} is
{}", t.as_csv());
}

fn main() {

/* testing merging row- and column-headers:

   let a = HashMap::from([("Apple", 1.1), ("Banana", 2.3), ("Cherries", 3.14)]);
   let b = HashMap::from([("Apple", 2.2), ("Durian", 9.7)]);
   let t1 = from_map("ARB", &a);
   print_table("t1", &t1);
   let t2 = from_map("BTC", &b);
   print_table("t2", &t2);
   let m = merge(&t1, &t2);
*/

   let arb = HashMap::from([("Apple", 1.1), ("Banana", 1.2)]);
   let btc = HashMap::from([("Apple", 53123.9), ("Banana", 54007.3)]);
   let t1 = from_map("ARB", &arb);
   let t2 = from_map("BTC", &btc);
   let m = merge(&t1, &t2, 0.0);
   print_table("Merged table", &m);
}
