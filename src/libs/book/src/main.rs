// we run the functional tests for the libs here

use book::{
   string_utils::functional_tests::runoff as stri,
   date_utils::functional_tests::runoff as d,
   err_utils::ErrStr,
   test_utils::{collate_results,mk_tests,Thunk::E}
}; 

// #[tokio::main]  
// async 
fn main() -> ErrStr<()> {
   let _ = collate_results("book",
      mk_tests("string_utils date_utils", vec![E(stri), E(d)]))?;
   Ok(())
}

