// some standard APY-calculation functoins

use crate::{
   list_utils::{parse_nums,tail},
   utils::{get_args,id}
};

pub fn compute_real_r(supply: f32, borrow: f32, net_r: f32, spew: bool) -> f32 {
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
   println!("\nGiven:");
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

pub fn fetch_spew_n_nums() -> (bool, Vec<f32>) {
   let args = get_args();
   let spew: bool = args.len() > 0 && args[0] == "--spew";
   let f = if spew { tail } else { id };
   let nums = parse_nums(f(args));
   (spew, nums)
}
