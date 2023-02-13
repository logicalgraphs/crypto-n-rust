// extract info about the FIN liquidity pools then reports on these data.

use std::fmt;

use book::{
   csv_utils::CsvWriter,
   file_utils::extract_date_and_body,
   html_utils::p,
   list_utils::ht,
   report_utils::{Mode, mk_mode, print_footer, print_top5s},
   string_utils::to_string,
   utils::get_args
};

use crypto::types::{
   percentage::Percentage,
   usd::USD
};

mod numbers;
use crate::numbers::{parse_usd,parse_percent,skip_percent_or_collecting};

#[derive(Debug,Clone)]
struct LP {
   name: String,
   volume: USD,
   apr: Percentage
}

impl fmt::Display for LP {
   fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
      write!(formatter, "LP {} volume: {}, APR: {}",
             self.name, self.volume, self.apr)
   }
}

impl CsvWriter for LP {
   fn as_csv(&self) -> String {
      format!("{},{},{}", self.name, self.volume, self.apr)
   }
}

fn usage() {
   println!("./lps <date> <mode> <lp-file>");
   println!("\n\twhere mode is {{text|html}}");
   println!("\nPrints the top-5s of the LPs by volume and APR.\n");
}

fn main() {
   let args = get_args();
   if let (Some(date), args1) = ht(args) {
      if let (Some(made), files) = ht(args1) {
         let mode = mk_mode(&made);
         fn title(kind: &str) -> String {
            format!("LPs on @TeamKujira BOW by {kind}")
         }
         for file in files {
            let (_, lines) = extract_date_and_body(file);
            let mut lps = process_lps(lines);
            let mut vols: Vec<LP> = lps.clone();
            vols.sort_by(|a, b| b.volume.partial_cmp(&a.volume).unwrap());
            print_top5s(&title("volume"), &date, &vols, &mode);
            print_100k(&mode);
            lps.sort_by(|a, b| b.apr.partial_cmp(&a.apr).unwrap());
            print_top5s(&title("APR(combined"), &date, &lps, &mode);
            print_footer(&mode, "src/ch08/lps", "lps");
         }
      }
      println!("Hi, mom!");
   } else {
      usage();
   }
}

fn print_100k(mode: &Mode) {
   let paragraph = if mode == &Mode::TEXT { to_string } else { p };
   let mesg = "Showing all LPs with $100k+ volume";
   println!("{}\n", paragraph(mesg))
}

fn process_lps(lines: Vec<String>) -> Vec<LP> {
   let mut lps: Vec<LP> = Vec::new();
   process_lp(lines, &mut lps);
   lps
}

fn process_lp(lines: Vec<String>, lps: &mut Vec<LP>) {
   let meat: Vec<String> =
      lines.into_iter().skip_while(|x| !x.contains('/')).collect();
   if !meat.is_empty() {
      if let (Some(lp), rest) = ht(meat) {
         let (vol, rest1) = parse_usd(&rest);
         let rest2 = skip_percent_or_collecting(&rest1);
         let (aprr, rest3) = parse_percent(&rest2);
         if let Ok(volume) = vol {
            if let Ok(apr) = aprr {
               lps.push(LP { name: lp, volume, apr });
            }
         }
         process_lp(rest3, lps);
      }
   }
}
