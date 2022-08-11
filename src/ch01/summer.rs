// be nice to somebody, or ... somebodies, you know?

use std::env;

fn main() {
   let args: Vec<_> = env::args().collect();
   let (_, args) = args.split_at(1);

   let nums: Vec<f32> = args.iter().map(|n| n.parse().unwrap()).collect();
   let sum: f32 = nums.iter().sum();
   println!("The sum of {:?} is {:?}", nums, sum);
}
