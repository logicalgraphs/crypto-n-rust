use reqwest::header::HeaderMap;

use crate::err_utils::{ErrStr,err_or};

/*
The skeleton upon which this get-fetch example is based is:

https://stackoverflow.com/questions/43222429/how-do-you-make-a-get-request-in-rust#:~:text=Sending%20a%20GET%20request%20is,send().unwrap()%3B%20assert_eq!

include:

reqwest = "0.9.18"

in the Cargo.toml-build-man&ifest
*/

/// a simple REST request-response
pub async fn read_rest(endpoint: &str) -> ErrStr<String> {
   let res = err_or(reqwest::get(endpoint).await, "https::GET")?;
   let body = err_or(res.text().await, "no body in Response")?;
   Ok(body)
}

/// When we need to send a REST request with headers
pub async fn read_rest_with(hm: HeaderMap, url: &str) -> ErrStr<String> {
   let client = reqwest::Client::new();
   let response = err_or(client
            .get(url)
            .headers(hm.clone())
            .send()
            .await, 
      &format!("Could not get a response from {url} with headers {hm:?}"))?;

   if response.status().is_success() {
            let body = err_or(response.text().await, "no text in response")?;
            Ok(body)
   } else {
      let status = response.status();
      let error_body = err_or(response.text().await, "no error in text")?;
      Err(format!("Error status: {status}; Error body: {error_body}"))
   }
}
