// stamps stdin (see: usage())

use std::io::stdin;
use chrono::prelude::*;

use book::{
   list_utils::head,
   utils::get_args
};

fn usage() {
   println!("echo <text> | ./stamp [date-string]\n");
   println!("\te.g.: echo blah | ./stamp 2049-blade-runner\n");
   println!("If [date-string] is not provided, the current date is stamped.");
}

fn main() {
   if let Some(dt) = head(get_args()) {
      if dt == "--help" || dt == "-h" {
         usage();
      } else {
         stamp(dt);
      }
   } else {
      let todaeg = Local::now().format("%Y-%m-%d");
      stamp(todaeg.to_string());
   }
}

fn stamp(dt: String) {
   println!("date: {}\n", dt);
   loop {
      let mut butter = String::new();
      stdin().read_line(&mut butter).expect("Done.");
      if butter == "" { break; }
      println!("{}", butter.trim());
   }
}
