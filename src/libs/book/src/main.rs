// we run the functional tests for the libs here

use book::{
   date_utils::functional_tests::runoff as d,
   file_utils::functional_tests::runoff as f,
   string_utils::functional_tests::runoff as stri,
   currency::usd::functional_tests::runoff as u,
   num::estimate::functional_tests::runoff as e,

   err_utils::ErrStr,
   test_utils::{collate_results,mk_tests,mk_sync}
}; 

fn main() -> ErrStr<()> {
   let tests = "date_utils file_utils string_utils currency::usd num::estimate";
   let _ = collate_results("book",
      & mut mk_tests(tests,
            vec![mk_sync(d), mk_sync(f), mk_sync(stri), 
                 mk_sync(u), mk_sync(e)]))?;
   Ok(())
}

