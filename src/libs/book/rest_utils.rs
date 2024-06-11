use crate::err_utils::{ErrStr,err_or};

/*
The skeleton upon which this get-fetch example is based is:

https://stackoverflow.com/questions/43222429/how-do-you-make-a-get-request-in-rust#:~:text=Sending%20a%20GET%20request%20is,send().unwrap()%3B%20assert_eq!

include:

reqwest = "0.9.18"

in the Cargo.toml-build-man&ifest
*/

pub async fn read_rest(endpoint: &str) -> ErrStr<String> {
   let res = err_or(reqwest::get(endpoint).await, "https::GET")?;
   let body = err_or(res.text().await, "no body in Response")?;
   Ok(body)
}

/* ----- test ---------------------------------------------------------------

fn usage() {
   println!("./burn");
   println!("\tReads data from a REST endpoint.");
}

// tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
// as a dependency in app Cargo.toml-file

#[tokio::main]
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
