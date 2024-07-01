use std::ops::Sub;

use chrono::{Days,NaiveDate};

use book::{
   err_utils::ErrStr,
   json_utils::AsJSON,
   list_utils::tail,
   string_utils::to_string,
   table_utils::{ingest,row_filter,col,rows},
   utils::get_args
};

use swerve::{fetch_pivots::fetch_lines,types::mk_emas};

fn usage() {
   println!("\n./answer12 <date> <days> <token1> <token2>");
   println!("\tSnarfs pivots.csv and ratios token1/token2 for <days>");
   println!("\tIt also computes the EMA20s for that token-pair.");
}

fn datef(s: &str) -> NaiveDate {
   NaiveDate::parse_from_str(s, "%Y-%m-%d")
             .expect(&format!("{s} not in date-format"))
}

fn parse_num(s: &str) -> f32 { 
   if s == "" { 0.0 } else {
      s.parse().expect(&format!("{s} is not a number"))
   }
}

#[tokio::main]
async fn main() -> ErrStr<()> {
   let args = get_args();
   if let [dat, dayz, token1, token2] = args.as_slice() {
      let date = datef(&dat);
      let days = Days::new(parse_num(&dayz) as u64);
      let start = date.sub(days);
      let pivs = fetch_lines().await?;
      doit(&tail(&pivs), &start, token1, token2);
   } else {
      usage();
   }
   Ok(())
}

fn doit(pivs: &Vec<String>, start: &NaiveDate, t1: &String, t2: &String) {
   let table = ingest(datef, to_string, parse_num, pivs, ",");
   fn in_range(d: &NaiveDate) -> impl Fn(&NaiveDate) -> bool + '_ {
      |date| { date.ge(d) }
   }
   let domain = row_filter(in_range(start), &table);

   let a = col(&domain, &t1).expect(&format!("NO TOKEN NAMED {t1}"));
   println!("{t1} data: {:?}", a);
   let b = col(&domain, &t2).expect(&format!("NO TOKEN NAMED {t2}"));
   println!("{t2} data: {:?}", b);

   let ratios: Vec<f32> =
      a.clone().into_iter()
               .zip(b.clone().into_iter())
               .map(|(a,b)| a / b)
               .collect();

   let dates = rows(&domain);
   jsonify(t1, t2, &dates, &ratios);
}

fn jsonify(t1: &str, t2: &str, dates: &Vec<NaiveDate>, ratios: &Vec<f32>) {
   let data = mk_emas(t1, t2, 20, dates, ratios);
   println!("emas = {};", data.as_json());
}
