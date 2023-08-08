use book::utils::get_args;

use meth::{
   stride::fetch_stride_lsds,
   types::print_lsds
};

fn usage() {
   println!("./lsd <date>");
   println!("\tReads data from a REST endpoint.");
}

fn main() -> Result<(), String> {
   if let Some(date) = get_args().first() {
      let lsds = fetch_stride_lsds()?;
      print_lsds(&date, &lsds);
   } else {
      usage();
   }
   Ok(())
}

