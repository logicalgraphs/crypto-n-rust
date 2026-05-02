use std::{
   collections::HashMap,
   fmt,
   pin::Pin
};

use futures::Future;

use tokio::runtime::Runtime;

use super::{
   err_utils::ErrStr,
   string_utils::{plural,words},
   utils::pred
}; 

pub type AsyncFn<RES> = Pin<Box<dyn Future<Output=ErrStr<RES>> + Send>>;
pub type SyncFn<T, RES> = Box<dyn Fn(T) -> ErrStr<RES>>;
pub enum Thunk<T, RES> { F(AsyncFn<RES>), E(SyncFn<T, RES>) }
use Thunk::*;

pub type Tests = HashMap<String, Thunk<(), usize>>;

pub fn mk_tests(names: &str, fns: Vec<Thunk<(), usize>>) -> Tests {
   words(names).into_iter().zip(fns.into_iter()).collect()
}

fn mk_null_sync(f: fn() -> ErrStr<usize>) -> SyncFn<(), usize> {
   Box::new(move |_: ()| f())
}

pub fn mk_sync(f: fn() -> ErrStr<usize>) -> Thunk<(), usize> {
   E(mk_null_sync(f))
}
pub fn mk_async<F: Future<Output=ErrStr<usize>> + Send + 'static>
      (res: F) -> Thunk<(), usize> {
   F(Box::pin(res))
}

pub fn same<T:PartialEq + fmt::Display>(a: T, b: T) -> ErrStr<usize> {
   pred(a == b, 1).ok_or(format!("{a} is not equal to {b}"))
}

pub fn run_test(test: &str, f: &mut Thunk<(), usize>) -> ErrStr<usize> {
   fn_preamble(test);
   let res = match f {
      E(f1) => f1(()),
      F(f2) => {
         let rt = Runtime::new().unwrap();
         // let boxed_future = Pin::into_inner(f2);
         // unfortunately, f2 is not an Unpin impl
         // block_on(boxed_future)
         rt.block_on(f2)
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
   preamble(suite);
   let (test_names, res) = run_all_tests(tests);
   report_test_results("book", &test_names, res)
}

fn fn_preamble(fn_name: &str) {
   println!("\n{fn_name} functional test\n");
}

pub fn preamble(module_name: &str) {
   println!("\n{module_name} functional tests\n");
}

pub type Function<T, RES> = Box<dyn Fn(T) -> RES>;
pub type Report<T, RES> =
   Box<dyn Fn(&str, T, Function<T, RES>) -> ErrStr<usize>>;

#[macro_export(local_inner_macros)]
macro_rules! create_testing {
    ($mod_name:expr) => {
        #[allow(unused_macros)]
        macro_rules! testing {
            ($fn_name:expr, $code:expr) => {{
                let name = format!("\n{}::run_{}", $mod_name, $fn_name);
                println!("{name} functional test\n");
                let _ = $code;
                println!("\n{name}:...ok");
                Ok(1)
            }};
        }

        #[allow(unused_macros)]
        macro_rules! report {
            ($fn_name:expr, $t:expr, $f:expr) => {{
                let name = format!("\n{}::run_{}", $mod_name, $fn_name);
                let res = $f($t.clone());
                println!("{name} functional test\n");
                println!("\tinput: {:?}, function: {}, result: {:?}",
                         $t, $fn_name, res);
                println!("\n{name}:...ok");
                Ok(1)
            }};
        }
    };
}

pub fn report_test_results(module_name: &str, test_names: &[String],
                           res: Vec<ErrStr<usize>>) -> ErrStr<usize> {
   if res.iter().all(Result::is_ok) {
      let res1: ErrStr<usize> = res.into_iter().sum();
      let len = res1.clone()?;
      let desig = if len == 1 { "The" } else { "All" };
      println!("\n{desig} {} passed.\n",
               plural(len, &format!("{module_name} functional test")));
      res1
   } else {
      failures(&res, &test_names)
   }
}

fn failures(res: &[ErrStr<usize>], tests: &[String])
      -> ErrStr<usize> {
   let len = tests.len();
   let fs: Vec<String> =
      res.iter()
         .enumerate()
         .filter_map(|(n,r)| pred(!r.is_ok(), tests[n].to_string()))
         .collect();
   let many = plural(fs.len(), &format!("/{len} functional test"));
   println!("The following {} FAILED!:\n\t{}", many, fs.join("\n\t"));
   Err(format!("{} FAILED!", many))
}

// ----- TESTS -------------------------------------------------------

// when you're testing the test-framework, you have reached test-Nirvana.

#[cfg(test)]
mod tests {
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

   fn passers() -> Vec<Thunk<(), usize>> {
      [a,b,c,d].into_iter().map(mk_sync).collect()
   }

   #[test]
   fn test_collate_results_ok() {
      let report =
         collate_results("test_utils", &mut mk_tests("a b c d", passers()));
      assert!(report.is_ok());
   }

   #[test]
   fn fail_collate_results() {
      let tests: Vec<Thunk<(), usize>> =
          vec![a,b,c,d,f].into_iter().map(mk_sync).collect();
      let report = collate_results("test_utils",
                                   &mut mk_tests("a b c d f", tests));
      assert!(report.is_err());
   }

   #[test]
   fn test_collate_results_async_ok() {
      let z = mk_async(zinc());
      let tests = vec![mk_sync(a),mk_sync(b),mk_sync(c),mk_sync(d),z];
      let report = collate_results("test_utils",
                                   &mut mk_tests("a b c d z", tests));
      assert!(report.is_ok());
   }

   #[test]
   fn fail_collate_results_async() {
      let z = mk_async(zinc());
      let zf = mk_async(thinc());
      let tests =
         vec![mk_sync(a),mk_sync(b),mk_sync(c),mk_sync(d),mk_sync(f),z,zf];
      let report =
         collate_results("test_utils",
                         &mut mk_tests("a b c d f z zf", tests));
      assert!(report.is_err());
   }
}

