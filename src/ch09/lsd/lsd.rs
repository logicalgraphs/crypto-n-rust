use book::utils::get_args;

use meth::{
   read_rest::read_rest,
   types::{parse_lsds, print_lsds}
};

fn usage() {
   println!("./lsd <date>");
   println!("\tReads data from a REST endpoint.");
}

fn main() {
   if let Some(date) = get_args().first() {
      let stroll = "Stride-Labs/stride/stakeibc/host_zone";
      let url = "https://stride-api.polkachu.com";
      match read_rest(&format!("{url}/{stroll}")) {
         Ok(body) => reportage(&body, date),
         err      => println!("ERROR! {err:?}")
      }
   } else {
      usage();
   }
}

fn reportage(body: &str, date: &str) {
   let lsds = parse_lsds(body);
   print_lsds(date, &lsds);
}
