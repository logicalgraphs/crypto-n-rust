// The 'using modules' example

mod utils;

fn main() {
   let nums = utils::get_nums();
   let sum: f32 = nums.iter().sum();
   println!("The sum of {:?} is {:?}", nums, sum);
}
