// extracts current market data from 
// https://api.kujira.app/api/coingecko/tickers

use std::collections::HashMap;

use crate::rest_utils::read_aliases;

// ----- Aliases -------------------------------------------------------

pub type Aliases = HashMap<String, String>;

pub fn alias(aliases: &Aliases, i: &String) -> String {
   aliases.get(i).unwrap_or(i).clone()
}

pub fn load_aliases(opt_url: &Option<String>) -> Aliases {
   let mut ans = HashMap::new();
   if let Some(url) = opt_url {
      let file = read_aliases(url).expect("Cannot read aliases file.");
      let all_lines: Vec<_> = file.split("\n").collect();
      let (_date, lines) = all_lines.split_at(3);

      for alias in lines {
         if let [id,name] = alias.split(",").collect::<Vec<_>>().as_slice() {
           ans.insert(id.to_string(), name.to_string());
         } else {
            if !alias.is_empty() {
               println!("Unable to parse alias: '{alias}'");
            }
         }
      }
   }
   ans
}
