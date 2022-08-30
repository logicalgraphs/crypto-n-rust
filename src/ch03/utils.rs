use std::env;

pub fn get_args() -> Vec<String> {
   let cmd_lin: Vec<String> = env::args().collect();
   let (_, args) = cmd_lin.split_at(1);
   args.to_vec()
}

pub fn get_nums() -> Vec<f32> {
   let nums: Vec<f32> =
      get_args().iter()
         .map(|n| n.parse().expect(&(n.to_owned() + " isn't a number")))
         .collect();
   nums
}
