use std::{
   collections::HashMap,
   fmt,
   fmt::Debug,
   pin::Pin
};

use futures::{Future,executor::block_on};

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
   let res = match f {
      E(f1) => f1(()),
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
   preamble(suite);
   let (test_names, res) = run_all_tests(tests);
   report_test_results("book", &test_names, res)
}

pub fn preamble(module_name: &str) {
   println!("\n{module_name} functional tests\n");
}

pub type Function<T, RES> = Box<dyn Fn(T) -> RES>;
pub type Report<T, RES> =
   Box<dyn Fn(&str, T, Function<T, RES>) -> ErrStr<usize>>;

pub fn reporter<T: Debug + Clone, RES: Debug>(module_name: String)
      -> Report<T, RES> {
   Box::new(move |test: &str, t: T, f: Function<T, RES>| {
      println!("\n{}::run_{test} functional test
      
        input: {:?}, function: {test}, result: {:?}

{}::{test}:...ok", module_name, t.clone(), f(t), module_name);
      Ok(1)
   })
}

pub fn bind<T, RES>(f: impl Fn(T) -> RES + 'static) -> Box<dyn Fn(T) -> RES> {
   Box::new(f)
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

