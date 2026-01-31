use std::{collections::HashMap,fmt};
use futures::{Future, executor::block_on};
use tokio::runtime::Runtime;

use super::{
   err_utils::ErrStr,
   string_utils::{plural,words},
   utils::pred
}; 

pub enum Thunk<T: Future<Output=ErrStr<()>>> { F(T), E(fn() -> ErrStr<()>) }
// pub enum Thunk { E(fn() -> ErrStr<()>) }
use Thunk::*;

pub type Tests<T> = HashMap<String, Thunk<T>>;

pub fn mk_tests<T: Future<Output=ErrStr<()>>>(names: &str, fns: Vec<Thunk<T>>)
      -> Tests<T> {
   words(names).into_iter().zip(fns.into_iter()).collect()
}

pub fn same<T:PartialEq + fmt::Display>(a: T, b: T) -> ErrStr<()> {
   pred(a == b, ()).ok_or(format!("{a} is not equal to {b}"))
}

fn run_test<T: Future<Output=ErrStr<()>>>(test: &str, f: Thunk<T>)
      -> ErrStr<()> {
   let res = match f { E(f1) => f1(), F(f2) => mk_sync::<T>()(f2) };
   println!("\n{test}:...{}",
	    if res.is_ok() { "ok" } else { "FAILURE!" });
   res
}

pub fn collate_results<T: Future<Output=ErrStr<()>>>
      (suite: &str, tests: &Tests<T>) -> ErrStr<()> {
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

pub fn mk_sync:<F: Future<Output=ErrStr<()>>>()
      -> impl Fn(F) -> ErrStr<()> {

   let rt = Runtime::new().expect("Failed to create Tokio runtime");

   move |future_to_run| {
      rt.block_on(future_to_run)
   }
}

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

#[cfg(test)]
mod tests {

// when you're testing the test-framework, you have reached test-Nirvana.

   use super::*;

   #[test]
   fn test_same_ok() {
      let res = same(1, 1);
      assert!(res.is_ok());
   }

   #[test]
   fn fail_same() {
      let res = same("same", "but different");
      assert!(res.is_err());
   }

   // test functions for the test functions ... NIRVANA!

   fn a() -> ErrStr<()> { Ok(()) }
   fn b() -> ErrStr<()> { Ok(()) }
   fn c() -> ErrStr<()> { Ok(()) }
   fn d() -> ErrStr<()> { Ok(()) }
   fn f() -> ErrStr<()> { Err("test f failed".to_string()) }

/*
   async fn zinc() -> ErrStr<()> { Ok(()) }
   async fn thinc() -> ErrStr<()> { Err("Failed; asynchronously!".to_string()) }
*/

   #[test]
   fn test_collate_results_ok() {
      let tests: Vec<Thunk<_>> =
         vec![a,b,c,d].into_iter().map(|f| E(f)).collect();
      let report =
         collate_results("test_utils", &mk_tests("a b c d", tests));
      assert!(report.is_ok());
   }

   #[test]
   fn fail_collate_results() {
      let tests: Vec<Thunk<_>> =
         vec![a,b,c,d,f].into_iter().map(|x| E(x)).collect();
      let report =
         collate_results("test_utils", &mk_tests("a b c d f", tests));
      assert!(report.is_err());
   }

/*
   #[test]
   fn test_collate_results_async_ok() {
      let z = mk_sync(zinc());
      let report =
         collate_results("test_utils", &mk_tests("a b c d z", vec![a,b,c,d,z]));
      assert!(report.is_ok());
   }

   #[test]
   fn fail_collate_results_async() {
      let z = mk_sync(zinc());
      let zf = mk_sync(thinc());
      let report =
         collate_results("test_utils",
            &mk_tests("a b c d f z zf", vec![a,b,c,d,f,z,zf]));
      assert!(report.is_err());
   }
*/
}

