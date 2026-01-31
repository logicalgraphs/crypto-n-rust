// we run the functional tests for the libs here
   
use book::{
   string_utils::{words, functional_tests::runoff as stri},
   err_utils::ErrStr,
   test_utils::{collate_results,run_test}
}; 

fn test_names() -> Vec<String> { words("string_utils") }

fn tests() -> Vec<(String, impl Fn() -> ErrStr<()>)> {
   let names = test_names();
   let tests_fs = [stri];
   names.into_iter().zip(tests_fs.into_iter()).collect()
}

// #[tokio::main]  
// async 
fn main() -> ErrStr<()> {
   let res: Vec<ErrStr<()>> =
      tests().iter().map(|(n,f)| run_test(n,f)).collect();
   collate_results(&res, &test_names())
}     

