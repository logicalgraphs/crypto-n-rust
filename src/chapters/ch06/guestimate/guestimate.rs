
use book::num_utils::parse_estimate;

use crypto::types::usd::USD;

fn main() {
   test_estimates();
   test_usd();
}

fn test_estimates() {
   let a = "1.23".to_string();
   let b = "1.19k".to_string();
   let fa = parse_estimate(&a).expect("can't estimate a");
   let fb = parse_estimate(&b).expect("can't estimate b");
   println!("a ({}) is {}", a, fa);
   println!("b ({}) is {}", b, fb);
}

fn test_usd() {
   let monay = "$1,234.56";
   let dollahz: USD = monay.parse().expect("was ist das?");
   println!("monay ({}) is {}", monay, dollahz);
}
