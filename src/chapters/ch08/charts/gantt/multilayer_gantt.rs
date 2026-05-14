// from a trading-path we create a gantt chart, ... well, the data for a
// gantt chart.

// The gantt chart is rendered here:
// https://observablehq.com/@aaronkyle/google-gantt

// We read in a path then compute the efficacy of trading on that path

use book::{
   list_utils::ht,
   utils::get_args
};

fn usage() {
   let csv = "<comma-separated path>";
   println!("./gantt {csv}");
   println!("\n\tprints a gantt chart of path");
}

// first of all, let's define the crypto types

#[derive(strum_macros::Display)]
enum CoinType {
   Triumvirate,
   Secundus,
   Stablecoin,
   Alt
}

fn mk_coin_type(s: &str) -> CoinType {
   match s {
      "KUJI" | "ATOM"               => CoinType::Triumvirate,
      "OSMO"                        => CoinType::Secundus,
      "axlUSDC" | "axlUSDT" | "USK" => CoinType::Stablecoin,
      _                             => CoinType::Alt
   }
}

fn mein_coin(s: &str, mein: &str) -> String {
   if s == mein { "asset".to_string() } else { mk_coin_type(s).to_string() }
}

fn main() {
   let paths = get_args();
   if !paths.is_empty() {
      for line in paths { // there should be one path?
         let path: Vec<&str> = line.split(',').collect();
         if let Some(mein) = path.first() {
            let mut indices: Vec<char> = vec!['a'];
            print_prelude();
            let mut parents: Vec<String> = Vec::new();
            let frist_tail = "null".to_string();
            print_tasks(&mut parents, &mut indices, mein, 
                        &path, 1, 50, frist_tail);
            print_closer();
         }
      }
   } else {
      usage();
   }
}

fn print_prelude() {
   let url = "https://observablehq.com/@aaronkyle/google-gantt";
   println!("goto:\n{url}
   function daysToMilliseconds(days) {{
        return days * 24 * 60 * 60 * 1000;
      }}
      function jan(d) {{
        return new Date(2022, 11, d);
      }}
      data.addRows([");
}

fn print_closer() {
   println!("]);");
}

fn print_tasks(prev: &mut Vec<String>, indices: &mut Vec<char>, mein: &str,
               nodes: &Vec<&str>, day: u8, done: u8, beck: String) {
   if let (Some(h), t) = ht(&nodes.to_vec()) {
      if h.starts_with('(') {
         let (_, tru) = h.split_at(1);
         indices.push('1');
         prev.push(beck.clone());
         let mut new_nodes = t.clone();
         new_nodes.insert(0, tru);
         print_tasks(prev, indices, mein, &new_nodes, day + 1, done, beck);
      } else {
         let (nextus, tmrrow) =
            print_task(indices, mein, h, prev, day, done, beck);
         print_tasks(prev, indices, 
                     mein, &t, tmrrow, 0, nextus);
      }
   }
}

fn print_task(indices: &mut Vec<char>, mein: &str, h: &str,
              prev: &mut Vec<String>, day: u8, comp: u8, back: String)
    -> (String, u8) {
   fn mk_day(d: u8) -> String {
      format!("jan({d})")
   }

   let mill = "daysToMilliseconds(1)";
   let prelude: String = indices.iter().collect();
   let nidx = format!("{prelude}");
   update_index(indices);

   let (nh, new_back, day_add) = pop_parens(indices, prev, &nidx, h, 0);
   let new_day = day + day_add;
   let today = mk_day(new_day);
   let tmrrow = mk_day(new_day + 1);
   let ct = mein_coin(&nh, mein);
   
   let front = format!("['{nidx}', '{nh}', '{ct}', {today}, {tmrrow}, {mill}");
   let baccus = mb_null(&back);
   println!("{front}, {comp}, {baccus}],");
   (new_back, new_day + 1)
}

fn update_index(indices: &mut Vec<char>) {
   if let Some(idx) = indices.pop() {
      indices.push((idx as u8 + 1) as char);
   } else {
      panic!("No index!");
   }
}

fn mb_null(s: &str) -> String {
   if s == "null" { s.to_string() } else { format!("'{s}'") }
}

fn pop_parens(indices: &mut Vec<char>, prev: &mut Vec<String>,
              beck: &String, h: &str, daeg: u8) -> (String, String, u8) {
   if let Some(h1) = h.strip_suffix(')') {
      if let Some(parent) = prev.pop() {
         indices.pop();
         let new_beck = format!("{beck},{parent}");
         pop_parens(indices, prev, &new_beck, h1, 1 + daeg)
      } else {
         panic!("Popped past first parent!")
      }
   } else {
     (h.to_string(), beck.to_string(), daeg)
   }
}
