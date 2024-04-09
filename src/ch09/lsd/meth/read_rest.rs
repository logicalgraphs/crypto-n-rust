use std::collections::HashMap;

use book::{
   csv_utils::parse_csv,
   err_utils::{ErrStr,err_or},
   rest_utils::read_rest,
   string_utils::to_string
};

pub fn fetch_burns() -> ErrStr<HashMap<String,u8>> {
   let lg_url = "https://raw.githubusercontent.com/logicalgraphs";
   let burn_dir = "crypto-n-rust/main/src/ch09/lsd/data/burn-rates.csv";
   let csv = err_or(read_rest(&format!("{lg_url}/{burn_dir}")),
                    "error reading REST endpoint")?;
   fn burn_f(row: Vec<String>) -> Result<(String, u8), String> {
      if let [name, _, c, _] = row.as_slice() {
         let count: u8 = c.parse().expect(&format!("{c} is not a number"));
         Ok((name.to_string(), count))
      } else {
         Err(format!("{row:?} is not CSV-parseable!"))
      }
   }
   let lines: Vec<String> = csv.lines().map(to_string).collect();
   let rows = parse_csv(1, burn_f, &lines)?;
   let burns: HashMap<String, u8> = rows.into_iter().collect();
   Ok(burns)
}
