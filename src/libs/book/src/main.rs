// we run the functional tests for the libs here
   
use book::{
   string_utils::{words, plural, functional_tests::runoff as stri},
   err_utils::ErrStr,
   utils::pred
}; 

fn tests() -> Vec<String> {
   words("string_utils")
}

// #[tokio::main]  
// async 
fn main() -> ErrStr<()> { 
   let res = [stri()];       
   let len = res.len();
   if res.iter().all(Result::is_ok) {
      println!("\nAll {} passed.", plural(len, "functional test"));
      Ok(())
   } else {
      failures(&res, len)
   }  
}     
   
fn failures(res: &[ErrStr<()>], len: usize) -> ErrStr<()> {
   let fs: Vec<String> =
      res.iter()
         .enumerate()
         .filter_map(|(n,r)| pred(!r.is_ok(), tests()[n].clone()))
         .collect();
   let many = plural(fs.len(), &format!("/{len} functional test"));
   println!("The following {} FAILED!:\n\t{}", many, fs.join("\n\t"));
   Err(format!("{} FAILED!", many))
}  

