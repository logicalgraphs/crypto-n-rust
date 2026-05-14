/*
use std::{
   collections::HashMap,
   fmt,
   pin::Pin
};

use futures::Future;

use tokio::runtime::Runtime;

use book::{
   err_utils::ErrStr,
   string_utils::{plural,words},
   test_utils::report_test_results,
   utils::pred
}; 

use linkme::distributed_slice;

use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, LitStr};

type RegisteredFn = fn() -> ErrStr<usize>;
type RegisteredName = &'static str;

#[distributed_slice]
pub static REGISTRY: [(RegisteredFn, RegisteredName)] = [..];

#[proc_macro]
pub fn create_functional_tests(input: TokenStream) -> TokenStream {
   let module_name = parse_macro_input!(input as LitStr).value();
    
   let expanded = quote! {
       #[allow(unused_macros)] macro_rules! testing {
       
          ($fn_name:expr, $code:expr) => {
              paste! {
                 let name = [<run_ $fn_name>];
                 fn $name() -> ErrStr<usize> {
                    let name = format!("\n{}::run_{}", $mod_name, $fn_name);
                    println!("{name} functional test\n");
                    let _ = $code;
                    println!("\n{name}:...ok");
                    Ok(1)
                 }
                 #[distributed_slice(REGISTRY)]
                 static __: RegisteredFn = ($name, stringify!($name));
              }
          };
       }
       #[allow(unused_macros)] macro_rules! run_all_functional_tests {
          () => {
             pub fn runoff() -> ErrStr<usize> {
                let mut fns = Vec::new();
                for (f,n) in REGISTRY {
                   let a = f();
                   fns.push((n.to_string(), a));
                }
                let (a, b): (Vec<_>, Vec<_>) = fns.into_iter().unzip();
                report_test_results(stringify!(#module_name), &a, b)
             }
          };
       }
    };

    TokenStream::from(expanded)
}
*/

