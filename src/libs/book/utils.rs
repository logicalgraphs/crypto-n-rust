use std::{ env::{args,var}, fmt::{Debug,Display} };

use futures::Future;

use tokio::runtime::Runtime;

use crate::{
   err_utils::{ErrStr,err_or},
   list_utils::tail
};

// ----- command line arguments fetch functions -------------------------

pub fn get_args() -> Vec<String> {
   let argus: Vec<String> = args().collect();
   tail(&argus)
}

// ----- env vars -------------------------------------------------------

pub fn get_env(variable: &str) -> ErrStr<String> {
   err_or(var(variable),
          &format!("Could not fetch {variable} var from environment"))
}

// ----- Category theory ------------------------------------------------

pub fn id<T: Clone>(t: T) -> T { t }
pub fn k<T:Clone,A>(t: T) -> impl Fn(A) -> T { move |_: A| t.clone() }

pub fn pred<T>(head: bool, consequence: T) -> Option<T> {
   if head { Some(consequence) } else { None }
}

pub fn composer<A, B, C>(g: impl Fn(B) -> C, f: impl Fn(A) -> B)
         -> impl Fn(A) -> C {
   move |a: A| g(f(a))
}

// ----- Synchronicity --------------------------------------------------

pub fn now<T, F: Future<Output = T>>(f2: F) -> T {
   let rt = Runtime::new().unwrap();
   rt.block_on(f2)
}

// ----- Result / Either ------------------------------------------------

pub fn resolve<T: Display, E: Debug>(x: Result<T, E>) -> String {
   match x {
      Ok(a) => format!("{a}"),
      Err(y) => panic!("Resolving result-type give error: {y:?}")
   }
}

// ----- DEBUG ----------------------------------------------------------

pub fn debug<T: Debug>(x: T) -> String { format!("{x:?}") }

// ----- Reference ------------------------------------------------------

pub fn deref<A, B, F>(f: F) -> impl Fn(A) -> B
      where F: Fn(&A) -> B {
   move |a: A| f(&a)
}

// ----- TESTS ----------------------------------------------------------

#[cfg(test)]
#[cfg(not(tarpaulin_include))]
mod tests {
   use super::*;

   #[test] fn test_now() {
      println!("now() executes an asynchronous block, ... well: now.");
      let fiver = now(async { 5 });
      assert_eq!(5, fiver);
   }

   #[test] fn test_id() { assert_eq!(6, id(6)); }
   #[test] fn test_pred_none() {
      assert_eq!(None, pred("foo" == "bar", "wut"));
   }
   #[test] fn test_pred_some() { assert_eq!(Some(1), pred("foo" == "foo", 1)); }
}

