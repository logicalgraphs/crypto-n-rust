use std::{
   collections::HashMap,
   fmt,
   pin::Pin
};

use futures::{Future,executor::block_on};

use super::{
   err_utils::ErrStr,
   string_utils::{plural,words},
   utils::pred
}; 

pub type AsyncFn = Pin<Box<dyn Future<Output=ErrStr<usize>> + Send>>;
pub enum Thunk { F(AsyncFn), E(fn() -> ErrStr<usize>) }
use Thunk::*;

pub type Tests = HashMap<String, Thunk>;

pub fn mk_tests(names: &str, fns: Vec<Thunk>) -> Tests {
   words(names).into_iter().zip(fns.into_iter()).collect()
}

pub fn mk_sync(f: fn() -> ErrStr<usize>) -> Thunk { E(f) }
pub fn mk_async<F: Future<Output=ErrStr<usize>> + Send + 'static>
      (res: F) -> Thunk {
   F(Box::pin(res))
}

pub fn same<T:PartialEq + fmt::Display>(a: T, b: T) -> ErrStr<usize> {
   pred(a == b, 1).ok_or(format!("{a} is not equal to {b}"))
}

pub fn run_test(test: &str, f: &mut Thunk) -> ErrStr<usize> {
   let res = match f {
      E(f1) => f1(),
      F(f2) => {
         // let boxed_future = Pin::into_inner(f2);
         // unfortunately, f2 is not an Unpin impl
         // block_on(boxed_future)
         block_on(f2)
      }
   };
   test_result(test, res)
}

pub fn test_result(test: &str, res: ErrStr<usize>) -> ErrStr<usize> {
   println!("\n{test}:...{}",
	    if res.is_ok() { "ok" } else { "FAILURE!" });
   res
}

fn run_all_tests(tests: &mut Tests) -> (Vec<String>, Vec<ErrStr<usize>>) {
   let mut test_names = Vec::new();
   let mut res = Vec::new();
   let _ = tests.retain(|test, thunk| {
      let ans = run_test(&test, thunk);
      test_names.push(test.to_string());
      res.push(ans);
      false
   });
   (test_names, res)
}

pub fn collate_results(suite: &str, tests: &mut Tests) -> ErrStr<usize> {
   println!("\n{suite} functional tests\n");
   let (test_names, res) = run_all_tests(tests);
   report_test_results(test_names, res)
}

pub fn report_test_results(test_names: Vec<String>, res: Vec<ErrStr<usize>>)
      -> ErrStr<usize> {
   let len = test_names.len();
   if res.iter().all(Result::is_ok) {
      let res1: ErrStr<usize> = res.into_iter().sum();
      let len = res1.clone()?;  // the real len
      let desig = if len == 1 { "The" } else { "All" };
      println!("\n{desig} {} passed.\n", plural(len, "functional test"));
      res1
   } else {
      failures(&res, &test_names, len)
   }  
}

fn failures(res: &[ErrStr<usize>], tests: &[String], len: usize)
      -> ErrStr<usize> {
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

   fn a() -> ErrStr<usize> { Ok(1) }
   fn b() -> ErrStr<usize> { Ok(1) }
   fn c() -> ErrStr<usize> { Ok(1) }
   fn d() -> ErrStr<usize> { Ok(1) }
   fn f() -> ErrStr<usize> { Err("test f failed".to_string()) }

   async fn zinc() -> ErrStr<usize> { Ok(1) }
   async fn thinc() -> ErrStr<usize> {
      Err("Failed; asynchronously!".to_string())
   }

   fn passers() -> Vec<Thunk> { [a,b,c,d].into_iter().map(mk_sync).collect() }

   #[test]
   fn test_collate_results_ok() {
      let report =
         collate_results("test_utils", &mut mk_tests("a b c d", passers()));
      assert!(report.is_ok());
   }

   #[test]
   fn fail_collate_results() {
      let tests: Vec<Thunk> = //  postpend(&passers(), mk_sync(f));
         // [passers().as_slice(), &[mk_sync(f)]].concat().as_vec();
          vec![a,b,c,d,f].into_iter().map(mk_sync).collect();
      let report = collate_results("test_utils",
                                   &mut mk_tests("a b c d f", tests));
      assert!(report.is_err());
   }

   #[test]
   fn test_collate_results_async_ok() {
      let z = mk_async(zinc());
      let tests = vec![E(a),E(b),E(c),E(d),z];
      let report = collate_results("test_utils",
                                   &mut mk_tests("a b c d z", tests));
      assert!(report.is_ok());
   }

   #[test]
   fn fail_collate_results_async() {
      let z = mk_async(zinc());
      let zf = mk_async(thinc());
      let tests = vec![E(a),E(b),E(c),E(d),E(f),z,zf];
      let report =
         collate_results("test_utils",
                         &mut mk_tests("a b c d f z zf", tests));
      assert!(report.is_err());
   }
}

