// Here we state our real APY, but also compute what we need to achieve our goal

mod utils;
mod apy_utils;

fn usage() {
   println!("\n$ ./apy_goal [--spew] <supply> <borrow> <net_apy>");
   println!("               <sAVAX price> <yield-per-day-goal>");
}

fn main() {
   let (spew, nums) = apy_utils:: fetch_spew_n_nums();
   if let [supply, borrow, net, s_avax, yield_goal] = nums.as_slice() {
      let real_r = 
         apy_utils::compute_real_r(*supply, *borrow, *net / 100.0, spew);
      goal_computer(*supply, *borrow, *net / 100.0, real_r, *yield_goal, *s_avax);
   } else {
     usage();
   }
}

fn goal_computer(supply: f32, borrow: f32, net_apy: f32, real_apy: f32,
                 yield_per_day: f32, s_avax_price: f32) {
   let goal_principal = yield_per_day / real_apy * 365.0;
   let goal_s_avax_p   = goal_principal / s_avax_price;
   let goal_leverage  = yield_per_day / net_apy * 365.0;
   let goal_s_avax_l   = goal_leverage / s_avax_price;

   let current_princ  = supply - borrow;
   let percent_princ  = current_princ / goal_principal * 100.0;
   let current_lever  = supply + borrow;
   let percent_lever  = current_lever / goal_leverage * 100.0;

   println!("Net APY on @BenqiFinance:       {}%", net_apy);
   println!("Computed Real APY on principal: {}%", real_apy * 100.0);
   println!("\nhttps://github.com/logicalgraphs/crypto-n-rust/blob/main/src/ch05/apy_goal.rs");
   println!("\nTo make ${}/day in yields, I need:", yield_per_day);
   println!("\n${} in principal or {} $sAVAX", goal_principal, goal_s_avax_p);
   println!("${} in leverage or {} $sAVAX", goal_leverage, goal_s_avax_l);
   println!("\nI am at {}% principal goal and {}% leverage goal.",
            percent_princ, percent_lever);
}
