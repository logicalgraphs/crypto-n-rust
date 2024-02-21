// reads a color palette file 

use rand::{
   seq::IteratorRandom,
   thread_rng
};

use std::{
   clone::Clone,
   fmt::Debug
};

use book::{
   file_utils::lines_from_file,
   string_utils::to_string
};

fn heador<T:Debug + Clone>(v: Vec<T>) -> Option<T> {
   v.first().cloned()
}

// from Kirill A. Khalitov on Stack Overflow
// https://stackoverflow.com/questions/45786955/how-to-compose-functions-in-rust

macro_rules! compose {
   ($f: expr) => {
      move |g: fn(_) -> _| move |x: _| $f(g(x))
   };
}

fn tabs(s: String) -> Vec<String> {
   s.split("\t").map(to_string).collect()
}

pub fn colors(file: &str, n: usize) -> Vec<String> {
   let mut rng = thread_rng();
   lines_from_file(file)
      .into_iter()
      .filter_map(compose!(heador)(tabs))
      .choose_multiple(&mut rng, n)
}
