// Given a marketplace and a set of path a -> ... -> z
// These algorithms experiment with these paths and sort to best (last) outcome.
// ... this is all very comonadic.

use std::collections::{HashSet};

use book::{
   file_utils::lines_from_file,
   list_utils::{head,tail,ht},
   string_utils::str_string
};

use crate::types::marketplace::{OrderBook,ratio_for};

// We start out with a higher-order function to print a path experiment:

pub fn print_path(ntoks: f32) -> impl Fn(&(f32, Vec<f32>, String)) -> () {
   move | p: &(f32, Vec<f32>, String) |  {
      let (ans, interms, line) = p;
      println!("For {}:", line);
      println!("    {:?}", interms);
      println!("\t{} tokens becomes {}.", ntoks, ans);
   }
}

// Here we do the work of processing a string-input to a path experiment

pub fn process_paths(ntoks: f32, market: &HashSet<OrderBook>)
   -> impl Fn(&String) -> Vec<(f32, Vec<f32>, String)> + '_ {
   move |file: &String| {
      let lines = lines_from_file(file);
      let mut paths: Vec<(f32, Vec<f32>, String)> = tail(lines).iter()
          .map(process_path(ntoks, market))
          .collect();
      paths.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
      paths
   }
}

pub fn process_paths_for<'a>(ntoks: f32, tok: &'a str,
                             market: &'a HashSet<OrderBook>)
   -> impl Fn(&String) -> Vec<(f32, Vec<f32>, String)> + 'a {
   move |file: &String| {
      let lines = lines_from_file(file);
      let paths_p: Option<Vec<(f32, Vec<f32>, String)>> = tail(lines).iter()
          .map(process_path_for(ntoks, tok, market))
          .collect();
      let mut paths = if let Some(p) = paths_p { p } else { Vec::new() };
      paths.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());
      paths
   }
}

pub fn process_path(ntoks: f32, market: &HashSet<OrderBook>)
   -> impl Fn(&String) -> (f32, Vec<f32>, String) + '_ {
   move |line: &String| {
      let mut interms: Vec<f32> = Vec::from([ntoks]);
      let path: Vec<String> = line.split(',').map(str_string).collect();
      let mut ans: f32 = ntoks;
      ans = process_books(ans, market, &mut interms, &path);
      (ans, interms.clone(), line.clone())
   }
}

pub fn process_path_for<'a>(ntoks: f32, tok: &'a str,
                            market: &'a HashSet<OrderBook>)
   -> impl Fn(&String) -> Option<(f32, Vec<f32>, String)> + 'a {
   move |line: &String| {
      let raw_path: Vec<&str> = line.split(',').collect();
      fn str_str_str(s: &&str) -> String {
         str_string(*s)
      }
      let path: Vec<String> =
         raw_path.iter().skip_while(|n| n != &&tok).map(str_str_str).collect();
      if path.is_empty() {
         None
      } else {
         let mut interms: Vec<f32> = Vec::from([ntoks]);
         let mut ans: f32 = ntoks;
         ans = process_books(ans, market, &mut interms, &path);
         Some((ans, interms.clone(), line.clone()))
      }
   }
}

// ----- HELPER FUNCTIONS ---------------------------------------------

// this function is a comonadic extension, and so demonstrates `experiment`

fn process_books(ntoks: f32, market: &HashSet<OrderBook>,
                 interms: &mut Vec<f32>, path: &Vec<String>) -> f32 {
   let mut ans: f32 = ntoks;
   if let (Some(from), tos) = ht(path.to_vec()) {
      if let Some(to) = head(tos.clone()) {
         let tot: f32 = ans * ratio_for(market, &from, &to);
         interms.push(tot);
         ans = process_books(tot, market, interms, &tos);
      }
   }
   ans
}

