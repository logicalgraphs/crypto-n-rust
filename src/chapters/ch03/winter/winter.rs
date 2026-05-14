// The 'using modules' example

use book::{
   list_utils::parse_nums_opt,
   utils::get_args
};

fn main() {
   let args = get_args();
   let nums = parse_nums_opt(args);
   let sum: f32 = nums.iter().sum();
   println!("The sum of {:?} is {}", nums, sum);
}
