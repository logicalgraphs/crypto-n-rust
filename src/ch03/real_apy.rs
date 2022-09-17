// Computes real APY from net APY on leveraged assets

mod utils;
mod apy_utils;

fn usage() {
   println!("\n$ ./real_apy [--spew] <supply> <borrow> <net_apy>");
   println!("\n\tcomputes real APY from leveraged assets.");
   println!("\nFor example:");
   println!("\n\t$ ./real_apy 17206.37 9742.39 1.81");
   println!("\n\tYour real APY is 6.39%");
   println!("\n(add \"--spew\" to see all the glorious calculations)\n");
}

fn main() {
   let (spew, nums) = apy_utils::fetch_spew_n_nums();
   if let [supply, borrow, net] = nums.as_slice() {
      let real_r =
         apy_utils::compute_real_r(*supply, *borrow, *net / 100.0, spew);
      println!("Your real APY is {}%", real_r * 100.0);
   } else {
      usage();
   }
}
