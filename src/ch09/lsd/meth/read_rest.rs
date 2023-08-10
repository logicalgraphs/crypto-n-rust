use std::{
   collections::HashMap,
   io::Read
};

use book::csv_utils::parse_csv;

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

pub fn fetch_burns() -> Result<HashMap<String,u8>, String> {
   let lg_url = "https://raw.githubusercontent.com/logicalgraphs";
   let burn_dir = "crypto-n-rust/assemblage/src/ch09/lsd/data/burn-rates.csv";
   let csv = read_rest(&format!("{lg_url}/{burn_dir}"))?;
   fn burn_f(row: &Vec<&str>) -> Result<Option<(String, u8)>, String> {
      if let [name, _, c, _] = row.as_slice() {
         let count: u8 = c.parse().expect(&format!("{c} is not a number"));
         Ok(Some((name.to_string(), count)))
      } else {
         Err(format!("{row:?} is not CSV-parseable!"))
      }
   }
   let mut lines = csv.lines();
   let rows = parse_csv(1, burn_f, &mut lines)?;
   let burns: HashMap<String, u8> = rows.into_iter().collect();
   Ok(burns)
}
