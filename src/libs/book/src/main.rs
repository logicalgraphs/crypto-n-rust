// we run the functional tests for the libs here

use book::{
   currency::usd::functional_tests::runoff as u,
   date_utils::functional_tests::runoff as d,
   file_utils::functional_tests::runoff as f,
   list_utils::functional_tests::runoff as l,
   num::estimate::functional_tests::runoff as e,
   num_utils::functional_tests::runoff as n,
   parse_utils::functional_tests::runoff as p,
   stream_utils::functional_tests::runoff as stre,
   string_utils::functional_tests::runoff as stri,
   table_utils::functional_tests::runoff as t,
   tuple_utils::functional_tests::runoff as tu,

   err_utils::ErrStr,
   test_utils::{collate_results,mk_tests,mk_sync,mk_async}
}; 

#[cfg(not(tarpaulin_include))]
fn main() -> ErrStr<()> {
   let t1 = "date_utils file_utils string_utils currency::usd num::estimate";
   let t2 = "parse_utils num_utils table_utils stream_utils tuple_utils";
   let t3 = "list_utils";
   let tests = format!("{t1} {t2} {t3}");
   let _ = collate_results("book",
      & mut mk_tests(&tests,
            vec![mk_sync(d), mk_sync(f), mk_sync(stri), mk_sync(u), mk_sync(e),
                 mk_sync(p), mk_sync(n), mk_sync(t), mk_async(stre()),
                 mk_sync(tu), mk_sync(l)]))?;
                
   Ok(())
}

