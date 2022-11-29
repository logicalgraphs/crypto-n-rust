
use book::num_utils::parse_estimate;

fn main() {
   let a = "1.23".to_string();
   let b = "1.19k".to_string();
   let fa = parse_estimate(&a).expect("can't estimate a");
   let fb = parse_estimate(&b).expect("can't estimate b");
   println!("a ({}) is {}", a, fa);
   println!("b ({}) is {}", b, fb);
}
