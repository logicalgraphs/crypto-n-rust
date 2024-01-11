use chrono::naive::NaiveDate;

use book::{
   list_utils::ht,
   string_utils::to_string
};

/*
In ORCA's 7-day trailing liquidations, finding dates is fun, ...

We have this:

SOL	2024-01-08 07:36:39	

or this:

2024-01-08 00:36:45	

Which are kinda the same, once you reduce the split to the last two members

The date is simply the first one.
*/

pub fn find_date(line: &str) -> Result<NaiveDate, String> {
   let stuff: Vec<String> = line.split_whitespace().map(to_string).collect();
   
   fn extract_date(words: &Vec<String>) -> Option<String> {
      if words.is_empty() {
         None
      } else {
         let (ans, rest) = ht(words);
         if words.len() == 2 {
            ans
         } else {
            extract_date(&rest)
         }
      }
   }

   if let Some(date) = extract_date(&stuff) {
      match NaiveDate::parse_from_str(&date, "%Y-%m-%d") {
         Ok(dt) => Ok(dt),
         _ => Err(format!("Could not parse date from {date}"))
      }
   } else {
      Err(format!("Could not parse date in line {line}"))
   }
}

