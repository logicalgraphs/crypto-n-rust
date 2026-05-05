pub mod utils;
pub mod csv_utils;
pub mod currency;
pub mod date_utils;
pub mod err_utils;
pub mod file_utils;
pub mod html_utils;
pub mod json_utils;
pub mod list_utils;
pub mod matrix_utils;
pub mod num_utils;
pub mod num;
pub mod parse_utils;
pub mod report_utils;
pub mod rest_utils;
pub mod stream_utils;
pub mod string_utils;
pub mod table_utils;
pub mod tuple_utils;
pub mod types;

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
            ($fn_name:expr, $descr:expr, $t:expr, $f:expr) => {
                run_with!(@impl $fn_name, format!("{}", $descr), $t, $f);
            };
            ($fn_name:expr, $t:expr, $f:expr) => {
                run_with!(@impl $fn_name, "".to_string(), $t, $f);
            };
            (@impl $fn_name:expr, $descr:expr, $t:expr, $f:expr) => {
                run_helper!($fn_name, $descr, {
                   let res = $f($t);
                   println!("\tinput: {:?}, function: {}, result: {}",
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

// from Kirill A. Khalitov on Stack Overflow
// https://stackoverflow.com/questions/45786955/how-to-compose-functions-in-rust

#[macro_export]
macro_rules! compose {
   ($f: expr) => {
      move |g: fn(_) -> _| move |x: _| $f(g(x))
   };
}

