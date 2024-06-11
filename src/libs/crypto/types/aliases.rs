// extracts current market data from 
// https://api.kujira.app/api/coingecko/tickers

use std::collections::HashMap;

use book::file_utils::read_file;

use crate::rest_utils::{fin_res,read_aliases};

// ----- Aliases -------------------------------------------------------

pub type Aliases = HashMap<String, String>;

pub fn alias(aliases: &Aliases, i: &String) -> String {
   aliases.get(i).unwrap_or(i).clone()
}

pub async fn load_aliases_graph() -> Aliases {
   load_aliases(&Some(fin_res("main", "aliases.csv"))).await
}

pub async fn load_aliases(opt_url: &Option<String>) -> Aliases {
   if let Some(url) = opt_url {
      let file = read_aliases(url).await.expect("Cannot read aliases file.");
      aliases_loader(&file)
   } else {
      HashMap::new()
   }
}

pub fn load_aliases_from_file(filename: &str) -> Aliases {
   let file = read_file(filename);
   aliases_loader(&file)
}

fn aliases_loader(file: &str) -> Aliases {
   let mut ans = HashMap::new();
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
   ans
}
