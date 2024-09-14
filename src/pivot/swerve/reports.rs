use chrono::NaiveDate;

use book::{
   csv_utils::CsvWriter,
   string_utils::plural
};

use crate::types::{Diffs,Price};

fn headers(row: &Vec<Price>) {
   let syms: Vec<String> =
      row.iter().map(|price| price.0.1.to_string()).collect();
   println!(",{}", syms.join(","));
}

pub fn one_row(date: &NaiveDate, row: &Vec<Price>) {
   let vals: Vec<String> = row.iter().map(|price| price.1.as_csv()).collect();
   println!("{date},{}\n", vals.join(","));
}

fn one_row_with_headers(date: &NaiveDate, row: &Vec<Price>) {
   headers(row);
   one_row(date, row);
}

pub fn portfolio_prices(date: &NaiveDate, row: &Vec<Price>) {
   println!("\nPortfolio tokens,,,\n,,,");
   println!("date,token,id,price");
   row.into_iter().for_each(|((id,sym),price)|
      println!("{date},{sym},{id},${}", price.as_csv()));
}

pub fn report_diffs((kind, diffs): &Diffs) -> String {
   let n = diffs.len();
   let verb = if n > 1 { "were" } else { "was" };
   format!("There {verb} {} {}: {}",
            plural(n, "token"), kind.as_csv(), diffs.join(", "))
}

pub fn report(date: &NaiveDate, row: &Vec<Price>, errs: &Option<Diffs>) {
   portfolio_prices(date, row);
   println!("\n... and as one line:\n");
   one_row_with_headers(date, row);
   if let Some(diffs) = errs {
      report_diffs(&diffs);
   }
}
