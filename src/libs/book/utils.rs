use std::env::args;

use crate::list_utils::{tail,parse_nums};

// command line arguments fetch functions

pub fn get_args() -> Vec<String> {
   tail::<String>(&args().collect())
}

pub fn get_nums() -> Vec<f32> {
   parse_nums(get_args())
}

// Category theory

pub fn id<T: Clone>(t: &T) -> T { t.clone() }

// from Kirill A. Khalitov on Stack Overflow
// https://stackoverflow.com/questions/45786955/how-to-compose-functions-in-rust

#[macro_export]
macro_rules! compose {
   ($f: expr) => {
      move |g: fn(_) -> _| move |x: _| $f(g(x))
   };
}

pub fn pred<T>(head: bool, consequence: T) -> Option<T> {
   if head { Some(consequence) } else { None }
}
