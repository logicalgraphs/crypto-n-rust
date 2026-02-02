// we run the functional tests for the libs here

use book::{
   string_utils::functional_tests::runoff as stri,
   date_utils::functional_tests::runoff as d,
   err_utils::ErrStr,
   test_utils::{collate_results,mk_tests,mk_sync}
}; 

// #[tokio::main]  
// async 
fn main() -> ErrStr<()> {
   let _ = collate_results("book",
      mk_tests("string_utils date_utils", vec![mk_sync(stri), mk_sync(d)]))?;
   Ok(())
}

