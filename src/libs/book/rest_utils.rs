use std::{
   io::Read,
   error::Error
};

/*
The skeleton upon which this get-fetch example is based is:

https://stackoverflow.com/questions/43222429/how-do-you-make-a-get-request-in-rust#:~:text=Sending%20a%20GET%20request%20is,send().unwrap()%3B%20assert_eq!

include:

reqwest = "0.9.18"

in the Cargo.toml-build-man&ifest
*/

pub fn read_rest(endpoint: &str) -> Result<String, Box<dyn Error>> {
   let mut body = String::new();
   let res = reqwest::get(endpoint);
   res?.read_to_string(&mut body)?;
   Ok(body)
}

/* ----- test ---------------------------------------------------------------

fn usage() {
   println!("./burn");
   println!("\tReads data from a REST endpoint.");
}

fn main() -> Result<(), String> {
   let body = read_orders("LOCAL_USK", 10)?;
   reportage(&body);
   usage();
   Ok(())
}

fn reportage(body: &str) {
   println!("I got {body}");
}

*/
