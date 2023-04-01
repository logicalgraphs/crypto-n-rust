// extract info about the FIN liquidity pools then reports on these data.

use std::fmt;

use book::{
   csv_utils::CsvWriter,
   file_utils::extract_date_and_body,
   list_utils::ht,
   report_utils::{mk_mode, print_footer, print_top, print_message},
   utils::get_args
};

use crypto::types::{
   usd::USD
};

use nums::{
   numbers::{parse_usd,parse_percent,parse_percent_or_collecting},
   percs::Perc
};

#[derive(Debug,Clone)]
struct LP {
   name: String,
   volume: USD,
   apr_21_day: Perc,
   apr_combined: Perc
}

impl fmt::Display for LP {
   fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
      write!(formatter,
             "LP {} volume: {}, APR(21 day trading): {}, APR(combined): {}",
             self.name, self.volume, self.apr_21_day, self.apr_combined)
   }
}

impl CsvWriter for LP {
   fn as_csv(&self) -> String {
      format!("{},{},{},{}", self.name, self.volume,
                             self.apr_21_day, self.apr_combined)
   }
}

fn usage() {
   println!("./lps <date> <mode> <lp-file>");
   println!("\n\twhere mode is {{text|html}}");
   println!("\nPrints the top-5s of the LPs by volume and APRs.\n");
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
            let (_, lines) = extract_date_and_body(&file);
            let mut lps = process_lps(lines);
            let mut vols: Vec<LP> = lps.clone();
            vols.sort_by(|a, b| b.volume.partial_cmp(&a.volume).unwrap());
            print_top(5, &title("volume"), &date, &vols, &mode);
            print_message(&mode, "Showing all LPs with $100k+ volume");
            lps.sort_by(
               |a, b| b.apr_combined.partial_cmp(&a.apr_combined).unwrap());
            print_top(5, &title("APR(combined)"), &date, &lps, &mode);
            print_message(&mode, "Showing all LPs with 100%+ APR/APY");
            lps.sort_by(
               |a, b| b.apr_21_day.partial_cmp(&a.apr_21_day).unwrap());
            print_top(5, &title("APR(21 Day Trading)"), &date, &lps, &mode);
            print_footer(&mode, "src/ch08/lps", "lps");
         }
      }
      println!("Hi, mom!");
   } else {
      usage();
   }
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
         let (apr_21_day, rest2) = parse_percent_or_collecting(&rest1);
         let (aprr, rest3) = parse_percent(&rest2);
         if let Ok(volume) = vol {
            if let Ok(apr_combined) = aprr {
               lps.push(LP { name: lp, volume, apr_21_day, apr_combined });
            }
         }
         process_lp(rest3, lps);
      }
   }
}
