// Computes real APY from net APY on leveraged assets

mod utils;

fn usage() {
   println!("\n$ ./real_apy [--spew] <supply> <borrow> <net_apy>");
   println!("\n\tcomputes real APY from leveraged assets.");
   println!("\nFor example:");
   println!("\n\t$ ./real_apy 17206.37 9742.39 1.81");
   println!("\n\tYour real APY is 6.39%");
   println!("\n(add \"--spew\" to see all the glorious calculations)\n");
}

fn main() {
   let mut args = utils::get_args();
   let spew: bool = args.len() > 0 && args[0] == "--spew";
   if spew { args.remove(0); }
   let nums = utils::parse_nums(args);
   if let [supply, borrow, net] = nums.as_slice() {
      let real_r = compute_real_r(*supply, *borrow, *net / 100.0, spew);
      println!("Your real APY is {}%", real_r * 100.0);
   } else {
      usage();
   }
}

fn compute_real_r(supply: f32, borrow: f32, net_r: f32, spew: bool) -> f32 {
   let principal = supply - borrow;
   let leverage = supply + borrow;
   let eoy_leverage = leverage * net_r.exp();
   let distribution = eoy_leverage - leverage;
   let eoy_principal = principal + distribution;
   let real_r = (eoy_principal / principal).ln();
   if spew { spewage(supply, borrow, net_r, principal, leverage, eoy_leverage,
                     distribution, eoy_principal); }
   real_r
}

fn spewage(supp: f32, borr: f32, rate: f32, princ: f32,
           lev: f32, eoyl: f32, dist: f32, eoyp: f32) {
   println!("Given:");
   println!("\tSupply (S):                               {}", supp);
   println!("\tBorrow (B):                               {}", borr);
   println!("\tRate (R):                                 {}%\n", rate * 100.0);
   println!("We compute:");
   println!("\tPrincipal (S - B = P):                    {}", princ);
   println!("\tLeverage  (S + B = L):                    {}", lev);
   println!("\tEnd-of-year leverage (L * exp(R) = EOYL): {}", eoyl);
   println!("\tDistribution (EOYL - L = D):              {}", dist);
   println!("\tEnd-of-year principal (P + D = EOYP):     {}", eoyp);
   println!("\t... and the real rate is ln(EOYP/P)\n");
}
