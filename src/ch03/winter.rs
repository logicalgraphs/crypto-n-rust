// Influenced by the following stacks:

// https://stackoverflow.com/questions/27043268/convert-a-string-to-int
// https://stackoverflow.com/questions/23100534/how-to-sum-the-values-in-an-array-slice-or-vec-in-rust

mod utils;

fn main() {
   let nums = utils::get_nums();
   let sum: f32 = nums.iter().sum();
   println!("The sum of {:?} is {:?}", nums, sum);
}
