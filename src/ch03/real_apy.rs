// Computes real APY from net APY on leveraged assets

mod utils;

fn usage() {
   println!("\n$ ./real_apy <supply> <borrow> <net_apy>");
   println!("\n\tcomputes real APY from leveraged assets.");
   println!("\nFor example:");
   println!("\n\t$ ./real_apy 17206.37 9742.39 1.18");
   println!("\n\tYour real APY is 6.39%\n");
}

fn main() {
   let nums = utils::get_nums();
   if let [supply, borrow, net] = nums.as_slice() {
      let real_r = compute_real_r(*supply, *borrow, *net / 100.0);
      println!("Your real APY is {}%", real_r * 100.0);
   } else {
      usage();
   }
}

fn compute_real_r(supply: f32, borrow: f32, net_r: f32) -> f32 {
   let principal = supply - borrow;
   let leverage = supply + borrow;
   let eoy_leverage = leverage * net_r.exp();
   let distribution = eoy_leverage - leverage;
   let eoy_principal = principal + distribution;
   let real_r = (eoy_principal / principal).ln();
   real_r
}
