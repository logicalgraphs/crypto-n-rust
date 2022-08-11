// be nice to somebody, or ... somebodies, you know?

use std::env;

fn main() {
   let args: Vec<_> = env::args().collect();
   let (_, names) = args.split_at(1);
    
   match names.len() {
     0 => { println!("Whom?"); }
     _ => {
         for name in names {
            println!("Hello, {}!", name);
         }
     }
   };
}
