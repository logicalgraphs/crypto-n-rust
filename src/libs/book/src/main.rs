// we run the functional tests for the libs here

use book::{
   string_utils::functional_tests::runoff as stri,
   date_utils::functional_tests::runoff as d,
   currency::usd::functional_tests::runoff as u,
   err_utils::ErrStr,
   test_utils::{collate_results,mk_tests,mk_sync}
}; 

fn main() -> ErrStr<()> {
   let _ = collate_results("book",
      & mut mk_tests("string_utils date_utils currency::usd",
                     vec![mk_sync(stri), mk_sync(d), mk_sync(u)]))?;
   Ok(())
}

