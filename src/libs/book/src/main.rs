// we run the functional tests for the libs here

use book::{
   date_utils::functional_tests::runoff as d,
   file_utils::functional_tests::runoff as f,
   num_utils::functional_tests::runoff as n,
   parse_utils::functional_tests::runoff as p,
   string_utils::functional_tests::runoff as stri,
   currency::usd::functional_tests::runoff as u,
   num::estimate::functional_tests::runoff as e,
   table_utils::functional_tests::runoff as t,
   stream_utils::functional_tests::runoff as stre,

   err_utils::ErrStr,
   test_utils::{collate_results,mk_tests,mk_sync,mk_async}
}; 

#[cfg(not(tarpaulin_include))]
fn main() -> ErrStr<()> {
   let t1 = "date_utils file_utils string_utils currency::usd num::estimate";
   let t2 = "parse_utils num_utils table_utils stream_utils";
   let tests = format!("{t1} {t2}");
   let _ = collate_results("book",
      & mut mk_tests(&tests,
            vec![mk_sync(d), mk_sync(f), mk_sync(stri), mk_sync(u), mk_sync(e),
                 mk_sync(p), mk_sync(n), mk_sync(t), mk_async(stre())]))?;
                
   Ok(())
}

