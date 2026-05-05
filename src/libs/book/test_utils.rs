use futures::Future;

use tokio::runtime::Runtime;

pub fn now<T, F: Future<Output = T>>(f2: F) -> T {
   let rt = Runtime::new().unwrap();
   rt.block_on(f2)
}

#[macro_export(local_inner_macros)]
macro_rules! create_testing {
    ($mod_name:expr) => {
        #[allow(unused_macros)] macro_rules! run {
            ($fn_name:expr, $test:expr, $code:expr) => {
               run!(@impl $fn_name, format!("{}", $test), $code);
            };
            ($fn_name:expr, $code:expr) => {
               run!(@impl $fn_name, "".to_string(), $code);
            };
            (@impl $fn_name:expr, $test_str:expr, $code:expr) => {
                run_helper!($fn_name, $test_str, $code);
            };
        }

        #[allow(unused_macros)] macro_rules! run_with {
            ($fn_name:expr, $t:expr, $f:expr) => {
                run_helper!($fn_name, "".to_string(), {
                   let res = $f($t);
                   println!("\tinput: {:?}, function: {}, result: {:?}",
                            $t, $fn_name, res);
                });
            };
        }
        #[allow(unused_macros)] macro_rules! run_helper {
           ($fn_name:expr, $descr:expr, $code:expr) => {
              paste! {
                 #[test] fn [<run_ $fn_name>]() -> ErrStr<()> {
                    let name = format!("{}::run_{}{}",
                                       $mod_name, $fn_name, $descr);
                    println!("{name} functional test\n");
                    let _ = $code;
                    println!("\n{name}:...ok\n");
                    Ok(())
                 }
              }
           };
        }
        #[allow(unused_macros)] macro_rules! run_all_functional_tests {
           () => { raft!("".to_string()); };
           ($descr:expr) => { raft!(format!("{}", $descr)); };
        }
        #[allow(unused_macros)] macro_rules! raft {
           ($descr:expr) => {
              #[test] pub fn runoff() {
                 println!("{} functional tests\n\n{}\n", $mod_name, $descr);
              }
           };
        }
    };
}

// ----- TESTS -------------------------------------------------------

#[cfg(test)]
#[cfg(not(tarpaulin_include))]
mod tests {
   use super::*;

   #[test] fn test_now() {
      println!("now() executes an asynchronous block, ... well: now.");
      let fiver = now(async { 5 });
      assert_eq!(5, fiver);
   }
}

