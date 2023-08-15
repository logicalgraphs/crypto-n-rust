use std::{
   collections::HashMap,
   io::Read
};

use book::csv_utils::{HashRow,CsvRowResult,HashedRowsResult,parse_csv};

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

pub fn fetch_burns() -> HashedRowsResult<u8> {
   let csv = read_rest(&burn_url("main"))?;
   fn burn_f(row: &Vec<&str>) -> CsvRowResult<HashRow<u8>> {
      if let [name, _, c, _] = row.as_slice() {
         let count = ubnd(c)?;
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

pub fn fetch_manual_lsds(date: &str) -> HashedRowsResult<ManualLSD> {
   fn man_f(row: &Vec<&str>) -> CsvRowResult<HashRow<ManualLSD> {
      if let [dt,lsd1,zne,rt,_url,burn] = row.as_slice() {
         let halted = dt != dated;
         let zone = zne.to_string();
         let lsd = lsd1.to_string();
         let rate: f32 = rt.parse.expect(&format!("{rt} is not a number"));
         let unbond = ubnd(burn)?;
         Ok(Some((lsd1.to_string(), ManualLSD { zone, lsd, rate, halted, unbond }))
      } else {
         Err(format!("{row:?} is not CSV-parseable!"))
      }
   }
}

fn ubnd(c: &str) -> Result<u8, String> {
   c.parse().expect(&format!("{c} is not a number"))
}

fn burn_url(branch: &str) -> String {
   let lg_url = "https://raw.githubusercontent.com/logicalgraphs";
   let burn_dir = format!("crypto-n-rust/{branch}/src/ch09/lsd/data/burn-rates.csv");
   format!("{lg_url}/{burn_dir}")
}
