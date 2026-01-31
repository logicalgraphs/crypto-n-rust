use std::fmt;
// use futures::executor::block_on;

use super::{
   err_utils::ErrStr,
   string_utils::plural,
   utils::pred
}; 

pub type TestResult = ErrStr<()>;

pub fn same<T:PartialEq + fmt::Display>(a: T, b: T) -> ErrStr<()> {
   pred(a == b, ()).ok_or(format!("{a} is not equal to {b}"))
}

fn report_test_result(test: &str, res: ErrStr<()>) -> ErrStr<()> {
   println!("Running test {test}:...{}",
	    if res.is_ok() { "ok" } else { "FAILURE!" });
   res
}

pub fn run_test(test: &str, f: impl Fn() -> ErrStr<()>) -> ErrStr<()> {
   let ans = f();
   report_test_result(test, ans)
}

pub fn collate_results(res: &[TestResult], test_names: &[String])
      -> ErrStr<()> {
   let len = res.len();
   if res.iter().all(Result::is_ok) {
      println!("\nAll {} passed.", plural(len, "functional test"));
      Ok(())
   } else {
      failures(&res, test_names, len)
   }  
}

/*
pub fn mk_sync(f: impl Fn() -> ErrStr<()>) -> impl Fn() -> ErrStr<()> {
   || { block_on(f()) }
}
*/

fn failures(res: &[ErrStr<()>], tests: &[String], len: usize) -> ErrStr<()> {
   let fs: Vec<String> =
      res.iter()
         .enumerate()
         .filter_map(|(n,r)| pred(!r.is_ok(), tests[n].to_string()))
         .collect();
   let many = plural(fs.len(), &format!("/{len} functional test"));
   println!("The following {} FAILED!:\n\t{}", many, fs.join("\n\t"));
   Err(format!("{} FAILED!", many))
}

