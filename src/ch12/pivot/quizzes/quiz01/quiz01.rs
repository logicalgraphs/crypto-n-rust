use book::err_utils::ErrStr;

fn usage() {
   println!("./quiz01");
   println!("\tReads data from a REST endpoint.");
}

fn main() -> ErrStr<()> {
   usage();

   // do something here to read the REST endpoint
   // URL: 
   Ok(())
}
