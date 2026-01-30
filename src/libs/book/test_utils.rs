use std::fmt::*;

use book::{
   err_utils::ErrStr,
   string_utils::{plural,words},
   utils::pred
}; 

pub type TestResult = ErrStr<()>;

fn same<T:PartialEq + fmt::Display>(a: T, b: T) -> ErrStr<()> {
   pred(a == b, ()).ok_or(format!("{a} is not equal to {b}))
}

fn report_test_result<I: fmt::Display, O: fmt::Display, R: fmt::Display>
      (test: &str, input: I, exp: R, output: O, res: ErrStr<()>) -> ErrStr<()> {
   println!("Running test {test}
	input: {input}
	expected: {exp}
	output: {ans}
	result: {}", if res.is_ok() { "ok" } else { "FAILURE!" });
   res
}

pub fn run_test<I: fmt::Display, R: fmt::Display + PartialEq>
      (test: &str, f: impl Fn(I) -> R, input: I, exp: R) -> ErrStr<()> {
   let ans = f(input);
   let res = same(exp, ans);
   report_test_result(test, input, exp, ans, res)
}

pub fn check_ok<I: fmt::Display, O>
      (test: &str, f: impl Fn(I) -> ErrStr<O>, input: I) -> ErrStr<()> {
   let ans = f(input);
   report_test_result(test, input, "Ok()", ans)
}

pub fn collate_results(res: &[TestResult], test_names: &[&str]) -> ErrStr<()> {
   let len = res.len();
   if res.iter().all(Result::is_ok) {
      println!("\nAll {} passed.", plural(len, "functional test"));
      Ok(())
   } else {
      failures(&res, len)
   }  
}
