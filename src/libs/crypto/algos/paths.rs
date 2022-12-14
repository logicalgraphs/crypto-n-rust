// Given a marketplace and a set of path a -> ... -> z
// These algorithms experiment with these paths and sort to best (last) outcome.
// ... this is all very comonadic.

use std::collections::{HashSet};

use book::{
   file_utils::lines_from_file,
   list_utils::{head,tail,ht}
};

use crate::types::{
   marketplace::{OrderBook,ratio_for},
   percentage::{mk_percentage}
};

// We start out with a higher-order function to print a path experiment:

pub fn print_path(ntoks: f32) -> impl Fn(&(f32, Vec<f32>, String)) -> () {
   move | p: &(f32, Vec<f32>, String) |  {
      let (ans, interms, line) = p;
      println!("For {line}:");
      println!("    {interms:?}");
      let roi = mk_percentage((ans - ntoks) / ntoks);
      println!("\t{ntoks} tokens becomes {ans}, {roi} ROI.");
   }
}

// Here we do the work of processing a string-input to a path experiment

pub fn paths_processor(f: &dyn Fn(&String) -> Option<(f32, Vec<f32>, String)>,
                       file: &String) -> Vec<(f32, Vec<f32>, String)> {
   let lines = lines_from_file(file);
   let mut paths: Vec<(f32, Vec<f32>, String)> =
      tail(lines).iter().filter_map(f).collect();
   paths.sort_by(|a, b| a.0.partial_cmp(&b.0)
        .expect(&format!("I don't know how to compare {a:?} and {b:?}")) );
   paths
}

pub fn process_with_path(ntoks: f32, market: &HashSet<OrderBook>,
   path: &Vec<String>) -> Option<(f32, Vec<f32>, String)> {
   if path.is_empty() {
      None
   } else {
      let mut interms: Vec<f32> = Vec::from([ntoks]);
      let ans: f32 = process_books(ntoks, market, &mut interms, path);
      nan_or_inf_or((ans, interms.clone(), path.join(",")))
   }
}

// ----- HELPER FUNCTIONS ---------------------------------------------

fn nan_or_inf_or(a: (f32, Vec<f32>, String))
   -> Option<(f32, Vec<f32>, String)> {
   if a.0.is_nan() || a.0.is_infinite() { None } else { Some(a.clone()) }
}

// process_books is a comonadic extension, and so demonstrates `experiment`

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
