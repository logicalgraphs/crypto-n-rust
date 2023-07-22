use book::utils::get_args;

use meth::{
   read_rest::read_rest_lsds,
   types::{parse_lsds_without_burn, LSD, print_lsds, mk_fake_lsd}
};

fn usage() {
   println!("./lsd <date>");
   println!("\tReads data from a REST endpoint.");
}

fn main() {
   if let Some(date) = get_args().first() {
      let stroll = "Stride-Labs/stride/stakeibc/host_zone";
      let url = "https://stride-api.polkachu.com";
      match read_rest_lsds(&format!("{url}/{stroll}")) {
         Ok(body) => reportage(&body, date),
         err      => println!("ERROR! {err:?}")
      }
   } else {
      usage();
   }
}

fn reportage(body: &str, date: &str) {
   let lsds = parse_lsds_without_burn(body);
   let fakers: Vec<LSD> = lsds.iter().map(mk_fake_lsd).collect();
   print_lsds(date, &fakers);
}
