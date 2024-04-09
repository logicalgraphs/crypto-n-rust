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
   compose,
   file_utils::parse_data,
   string_utils::to_string
};

fn heador<T:Debug + Clone>(v: Vec<T>) -> Result<T, String> {
   v.first().ok_or(format!("No first element of {v:?}")).cloned()
}

fn tabs(s: String) -> Vec<String> {
   s.split("\t").map(to_string).collect()
}

pub fn colors(file: &str, n: usize) -> Result<Vec<String>, String> {
   let mut rng = thread_rng();
   let colours: Vec<String> =
      parse_data(compose!(heador)(tabs), file, None)?;
   Ok(colours.into_iter().choose_multiple(&mut rng, n))
}
