// Influenced by the following stacks:

// https://stackoverflow.com/questions/27043268/convert-a-string-to-int
// https://stackoverflow.com/questions/23100534/how-to-sum-the-values-in-an-array-slice-or-vec-in-rust

use std::env;

fn main() {
   let args: Vec<_> = env::args().collect();
   let (_, args) = args.split_at(1);

   let nums: Vec<f32> =
          args.iter()
              .map(|n| n.parse().expect(&(n.to_owned() + " isn't a number")))
              .collect();
   let sum: f32 = nums.iter().sum();
   println!("The sum of {:?} is {:?}", nums, sum);
}
