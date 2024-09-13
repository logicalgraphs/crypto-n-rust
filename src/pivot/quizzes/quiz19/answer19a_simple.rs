// simple case: we take two tables and merge them, easy-peasies!

use std::{collections::HashMap,fmt::Display};
use book::{
   csv_utils::CsvWriter,
   err_utils::ErrStr,
   string_utils::to_string,
   table_utils::{Table,from_map,merge,merge_with_default_d,default_f},
   tuple_utils::first
};

fn print_table<A: Display + Clone,B: Display + Clone,C: Display>(name: &str,
     t: &Table<A,B,C>) {
   println!("{name} is
{}", t.as_csv());
}

type Packet<'a> = (&'a str, Vec<(&'a str, f32)>);
type LeTabl = Table<String, String, f32>;

fn from_packet<'a>(a: &Packet<'a>) -> LeTabl {
   fn firstly<'a>(x: &Packet<'a>) -> HashMap<String, f32> {
      x.1.iter().cloned().map(first(to_string)).collect()
   }
   let a1 /* steak sauce */ = firstly(a);
   let t1 = from_map(a.0, &a1);
   print_table(a.0, &t1);
   t1
}

fn merge_tables(t1: &LeTabl, t2: &LeTabl) -> ErrStr<LeTabl> {
   let m = merge_with_default_d(&t1, &t2, default_f(&0.0), true)?;
   print_table("Merged table", &m);
   println!("");
   Ok(m)
}

fn testing<'a>(t: &'a str, a: &Packet<'a>, b: &Packet<'a>) -> ErrStr<LeTabl> {
   println!("Testing {t}");
   let t1 = from_packet(&a);
   let t2 = from_packet(&b);
   merge_tables(&t1, &t2)
}

fn main() -> ErrStr<()> {
   let a = testing("merging disparate row- and column-headers",
                   &("ARB",
             [("Apple", 1.1), ("Banana", 2.3), ("Durian", 3.14)].to_vec()),
                   &("BTC", [("Apple", 2.2), ("Cherries", 9.7)].to_vec()))?;

   println!("Testing merging a merged table with a new table");
   let t1 =
      from_packet(&("ETH", [("Banana", 2341.2), ("Elderberry", 2295.5),
                            ("Figs", 2401.1)].to_vec()));
   let _test_result = merge_tables(&a, &t1);

   let result = testing("merging disparate rows, same columns",
           &("Apple", [("ARB", 1.1), ("BTC", 53123.9)].to_vec()),
           &("Banana", [("ARB", 1.2), ("BTC", 54007.3)].to_vec()))
     .and_then(|t| {
        println!("Testing fail-on-merge with BTC data missing");
        let t2 = from_packet(&("Cherries", [("ARB", 0.87)].to_vec()));
        merge(&t, &t2)
   });
   match result {
      Ok(t) => Err(format!("ERROR! Tables merged with missing data! table:\n{}",
                           t.as_csv())),
      Err(msg) => {
         println!("SUCCESSFUL failed merge with missing data:\n * {msg}");
         Ok(())
      }
   }
}
