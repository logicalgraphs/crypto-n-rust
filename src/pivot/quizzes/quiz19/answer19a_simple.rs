// simple case: we take two tables and merge them, easy-peasies!

use std::{collections::HashMap,fmt::Display};
use book::{
   csv_utils::CsvWriter,
   string_utils::to_string,
   table_utils::{Table,from_map,merge_d},
   tuple_utils::first
};

fn print_table<A: Display + Clone,B: Display + Clone,C: Display>(name: &str,
     t: &Table<A,B,C>) {
   println!("{name} is
{}", t.as_csv());
}

type Packet<'a> = (&'a str, Vec<(&'a str, f32)>);

fn testing<'a>(test: &'a str, a: &Packet<'a>, b: &Packet<'a>) {
   println!("Testing {test}");
   fn firstly<'a>(x: &Packet<'a>) -> HashMap<String, f32> {
      x.1.iter().cloned().map(first(to_string)).collect()
   }
   let a1 /* steak sauce */ = firstly(a);
   let b1 = firstly(b);
   let t1 = from_map(a.0, &a1);
   print_table(a.0, &t1);
   let t2 = from_map(b.0, &b1);
   print_table(b.0, &t2);
   let m = merge_d(&t1, &t2, 0.0, true);
   print_table("Merged table", &m);
   println!("");
}

fn main() {
   testing("merging disparate row- and column-headers",
           &("ARB",
             [("Apple", 1.1), ("Banana", 2.3), ("Cherries", 3.14)].to_vec()),
           &("BTC", [("Apple", 2.2), ("Durian", 9.7)].to_vec()));

   testing("merging disparate rows, same columns",
           &("Apple", [("ARB", 1.1), ("BTC", 53123.9)].to_vec()),
           &("Banana", [("ARB", 1.2), ("BTC", 54007.3)].to_vec()));
}
