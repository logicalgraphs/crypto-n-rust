use std::{
   collections::HashMap,
   iter::zip
};

use book::string_utils::to_string;

pub fn extract_keys_symbols(pivots: &Vec<String>) -> HashMap<String, String> {
   let ids = pivots[0].split(",").skip(1).map(to_string);
   let syms = pivots[1].split(",").skip(1).map(to_string);
   zip(ids, syms).collect()
}