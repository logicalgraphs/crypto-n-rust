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
   Stablecoin,
   Alt
}

fn mk_coin_type(s: &str) -> CoinType {
   match s {
      "KUJI" | "ATOM" | "OSMO"      => CoinType::Triumvirate,
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
            print_prelude();
            print_tasks(None, 'a', mein, &path, 1);
            print_closer();
         }
      }
   } else {
      usage();
   }
}

fn print_prelude() {
   let url = "https://observablehq.com/@aaronkyle/google-gantt";
   println!("goto: {url}\n");
   println!("function daysToMilliseconds(days) {{
        return days * 24 * 60 * 60 * 1000;
      }}
      function dec(d) {{
        return new Date(2022, 11, d);
      }}
      data.addRows([");
}

fn print_closer() {
   println!("]);");
}

fn print_tasks(prev: Option<char>, idx: char, mein: &str,
               nodes: &Vec<&str>, day: u8) {
   if let (Some(h), t) = ht(nodes.to_vec()) {
      print_task(idx, mein, h, prev, day);
      print_tasks(Some(idx), (idx as u8 + 1) as char, mein, &t, day + 1);
   }
}

fn print_task(idx: char, mein: &str, h: &str, prev: Option<char>, day: u8) {
   fn mk_day(d: u8) -> String {
      format!("dec({d})")
   }

   let today = mk_day(day);
   let tmrrow = mk_day(day + 1);
   let mill = "daysToMilliseconds(1)";
   let ct = mein_coin(h, mein);
   let front = format!("['{idx}', '{h}', '{ct}', {today}, {tmrrow}, {mill},");
   let back = match prev {
      None => "10, null".to_string(),
      Some(x) => format!("0, '{x}'")
   };
   println!("{front} {back}],");
}
