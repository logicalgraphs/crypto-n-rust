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

// -- Iterative Development ---------------------------------------------------

#[macro_export] macro_rules! not_implemented {
    ($reason:literal) => {
        panic!("Not Implemented: {}", $reason)
    };
    ($reason:literal, $($arg:expr),* $(,)?) => {{
        // The $(let _ = $arg;)* block acts as a value-sink for any number 
        // of expressions of any type before panicking.
        $(let _ = $arg;)*
        panic!("Not Implemented: {}", $reason)
    }};
}

// -- Category Theory ---------------------------------------------------------

// from Kirill A. Khalitov on Stack Overflow
// https://stackoverflow.com/questions/45786955/how-to-compose-functions-in-rust

#[macro_export] macro_rules! compose {
   ($f: expr) => { move |g: fn(_) -> _| move |x: _| $f(g(x)) };
}

// -- Testing Framework -------------------------------------------------------

#[macro_export] macro_rules! cond {
   (if true { $($t:tt)* } else { $($f:tt)* }) => { $($t)* };
   (if false { $($t:tt)* } else { $($f:tt)* }) => { $($f)* };
}

#[macro_export] macro_rules! create_testing {
   ($mod_name:expr) => {
      create_testing!(@impl $mod_name, "".to_string(), false);
   };
   ($mod_name:expr, $descr:expr) => {
      create_testing!(@impl $mod_name, ::std::format!("{}", $descr), false);
   };
   ($mod_name:expr, $descr:expr, $is_app:tt) => {
      create_testing!(@impl $mod_name, ::std::format!("{}", $descr), $is_app);
   };
   (@impl $mod_name:expr, $description:expr, $is_app:tt) => {
      #[test] fn runoff() {
         let d = if $description.is_empty() {
            "".to_string()
         } else {
            ::std::format!("\n{}\n", $description)
         };
         ::std::println!("{} functional tests\n{d}", $mod_name);
      }
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
      $crate::cond! { if $is_app { run_helper!("usage", "", usage());
      } else { } }
   };
}
