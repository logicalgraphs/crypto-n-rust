use std::env;

// command line arguments fetch functions

pub fn get_args() -> Vec<String> {
   let cmd_lin: Vec<String> = env::args().collect();
   let (_, args) = ht::<String>(cmd_lin);
   args
}

pub fn get_nums() -> Vec<f32> {
   parse_nums(get_args())
}

// parse_nums() Influenced by the following overflows:

// https://stackoverflow.com/questions/27043268/convert-a-string-to-int
// https://stackoverflow.com/questions/23100534/how-to-sum-the-values-in-an-array-slice-or-vec-in-rust

pub fn parse_nums(strs: Vec<String>) -> Vec<f32> {
   strs.iter()
       .map(|n| n.parse().expect(&(n.to_owned() + " isn't a number")))
       .collect()
}

// list functions

pub fn ht<T: std::clone::Clone>(list: Vec<T>) -> (Option<T>, Vec<T>) {
   let (f, t) = list.split_at(1);
   (f.to_vec().pop(), t.to_vec())
}
