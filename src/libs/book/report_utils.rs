// common functions to generate reports

use crate::{
   csv_utils::CsvWriter,
   html_utils::{a,h,ol,p},
   utils::id
};

#[derive(PartialEq)]
pub enum Mode { HTML, TEXT }

pub fn mk_mode(m: &str) -> Mode {
   match m.to_lowercase().as_str() {
      "html" => Mode::HTML,
      "text" => Mode::TEXT,
      _      => panic!("Do not know the mode {}", m)
   }
}

pub fn print_footer(mode: &Mode, src_dir: &str, program: &str) {
   let repo = "https://github.com/logicalgraphs/crypto-n-rust/blob/main";
   let source = format!("{src_dir}/{program}.rs");
   let url = format!("{repo}/{source}");
   let prelude = "Report generated by";
   let prog = format!("./{program}");
   let blurb = if mode == &Mode::HTML {
      let sub_url = a(&url, &prog);
      p(&format!("{prelude} {sub_url}"))
   } else {
      format!("{prelude} {prog}:\n{url}")
   };
   println!("{blurb}");
}

fn spacer() -> String {
   p("&nbsp;")
}

fn h2(s: String) -> String {
   format!("{}\n{}", h(2, &s), spacer())
}

pub fn print_top5s<T: CsvWriter>(title: &str, date: &str, lps: &Vec<T>, 
      mode: &Mode) {
   let headerf = if mode == &Mode::HTML { h2 } else { id };
   let listerf = if mode == &Mode::HTML { ol } else { list };
   let header = format!("Top 5 {title}, {date}");
   println!("{}\n", headerf(header));
   let stringy: Vec<String> = lps.iter().map(|a| a.as_csv()).take(5).collect();
   println!("{}\n", listerf(&stringy));
}

fn list(l: &Vec<String>) -> String {
   let mut i = 1;
   let mut ans = String::new();
   for x in l {
      ans = format!("{ans}\n{i}. {x}");
      i += 1;
   }
   ans
}
