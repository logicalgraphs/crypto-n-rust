use book::{
   csv_utils::CsvWriter,
   string_utils::plural
};

use crate::types::{Diffs,Price,Token};

fn one_row(date: &str, row: &Vec<Price>) {

   // first we show the symbols as a header:
   let syms: Vec<Token> =
      row.iter().map(|price| price.0.1.to_string()).collect();
   println!(",{}", syms.join(","));

   // then we print their prices:
   let vals: Vec<String> = row.iter().map(|price| price.1.as_csv()).collect();
   println!("{date},{}\n", vals.join(","));
}

fn portfolio_prices(date: &str, row: &Vec<Price>) {
   println!("date,token,id,price");
   row.into_iter().for_each(|((id,sym),price)|
      println!("{date},{sym},{id},{}", price.as_csv()));
}

fn report_diffs((kind, diffs): &Diffs) {
   let n = diffs.len();
   let verb = if n > 1 { "were" } else { "was" };
   println!("There {verb} {} {}: {}",
            plural(n, "token"), kind.as_csv(), diffs.join(", "));
}

pub fn report(date: &str, row: &Vec<Price>, errs: &Option<Diffs>) {
   println!("\nPortfolio tokens,,,\n,,,");
   portfolio_prices(date, row);
   println!("\n... and as one line:\n");
   one_row(date, row);
   if let Some(diffs) = errs {
      report_diffs(&diffs);
   }
}
