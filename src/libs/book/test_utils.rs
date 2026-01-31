use std::{collections::HashMap,fmt};
// use futures::executor::block_on;

use super::{
   err_utils::ErrStr,
   string_utils::{plural,words},
   utils::pred
}; 

pub type Tests = HashMap<String, fn() -> ErrStr<()>>;

pub fn mk_tests(names: &str, fns: Vec<fn() -> ErrStr<()>>) -> Tests {
   words(names).into_iter().zip(fns.into_iter()).collect()
}

pub fn same<T:PartialEq + fmt::Display>(a: T, b: T) -> ErrStr<()> {
   pred(a == b, ()).ok_or(format!("{a} is not equal to {b}"))
}

fn run_test(test: &str, f: impl Fn() -> ErrStr<()>) -> ErrStr<()> {
   let res = f();
   println!("{test}:...{}",
	    if res.is_ok() { "ok" } else { "FAILURE!" });
   res
}

pub fn collate_results(suite: &str, tests: &Tests) -> ErrStr<()> {
   let len = tests.len();
   println!("\n{suite} functional tests\n");
   let res: Vec<ErrStr<()>> =
      tests.into_iter().map(|(k,v)| run_test(k, v)).collect();
   let test_names: Vec<String> =
      tests.keys().into_iter().map(String::to_string).collect();
   if res.iter().all(Result::is_ok) {
      let desig = if len == 1 { "The" } else { "All" };
      println!("\n{desig} {} passed.\n", plural(len, "functional test"));
      Ok(())
   } else {
      failures(&res, &test_names, len)
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

