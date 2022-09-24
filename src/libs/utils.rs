use std::env::args;

use crate::list_utils::{tail,parse_nums};

// command line arguments fetch functions

pub fn get_args() -> Vec<String> {
   tail::<String>(args().collect())
}

pub fn get_nums() -> Vec<f32> {
   parse_nums(get_args())
}

// Category theory

pub fn id<T>(t: T) -> T { t }
