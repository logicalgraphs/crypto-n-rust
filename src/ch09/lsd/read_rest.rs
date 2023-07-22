use std::io::Read;

/* 
The skeleton upon which this get-fetch example is based is:

https://stackoverflow.com/questions/43222429/how-do-you-make-a-get-request-in-rust#:~:text=Sending%20a%20GET%20request%20is,send().unwrap()%3B%20assert_eq!
*/

pub fn read_rest(url: &str) -> Result<String, String> {
   let mut res = reqwest::get(url)
                       .expect(&format!("Could not fetch URL {url}"));
   let mut body = String::new();
   res.read_to_string(&mut body).expect("Could not read fetch body");
   Ok(body)
}
