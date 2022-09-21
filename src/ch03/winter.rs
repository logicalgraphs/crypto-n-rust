// The 'using modules' example

use book::utils::get_nums;

fn main() {
   let nums = get_nums();
   let sum: f32 = nums.iter().sum();
   println!("The sum of {:?} is {:?}", nums, sum);
}
