use std::env::{args,var};

use crate::{
   err_utils::{ErrStr,err_or},
   list_utils::{tail,parse_nums}
};

// ----- command line arguments fetch functions -------------------------

pub fn get_args() -> Vec<String> {
   let argus: Vec<String> = args().collect();
   tail(&argus)
}

pub fn get_nums() -> Vec<f32> {
   parse_nums(get_args())
}

// ----- env vars -------------------------------------------------------

pub fn get_env(variable: &str) -> ErrStr<String> {
   err_or(var(variable),
          &format!("Could not fetch {variable} var from environment"))
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

