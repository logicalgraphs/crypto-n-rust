use std::io::stdin;

use book::{
   err_utils::{ErrStr,err_or},
   html_utils::{AsHTML,mk_table,attrib,TR,colspan,blank_cols,
                COL,p,COL::TD,mk_tr},
   list_utils::ht,
   matrix_utils::{Matrix,column_view},
   string_utils::to_string,
   utils::get_args
};

use crypto::types::usd::USD;

fn main() -> ErrStr<()> {
   let mut lines: Matrix<String> = Vec::new();
   loop {
      let mut butter = String::new();
      stdin().read_line(&mut butter).expect("EOF");
      if butter == "" { break; }
      lines.push(butter.trim().split("\t").map(to_string).collect());
   }
   if lines.is_empty() { usage() } else { deuceage(&lines) }
}

fn usage() -> ErrStr<()> {
   println!("./tabla: enHTMLTableifies rows of data.\n");
   println!("usage:\n");
   println!("$ echo <TSV data> | ./tabla [--totals [col1 [col2] [col3...]]]\n");
   println!("Spits out an HTML table of the <TSV data> with an optional");
   println!("footer row of the totals.");
   Ok(())
}

fn deuceage(lines: &Matrix<String>) -> ErrStr<()> {
   let footer = if let (Some(_totals), cols) = ht(&get_args()) {
      fn n(c: &String) -> Option<usize> {
         err_or(c.parse(), &format!("Cannot parse number {c}")).ok()
      }
      let mut totals: Vec<usize> = cols.iter().filter_map(n).collect();
      totals.sort();
      Some(totals_of(&totals, lines)?)
   } else { None };
   println!("{}", mk_table(&lines, footer).as_html());
   Ok(())
}

fn totals_of(cols_to_sum: &Vec<usize>, matrix: &Matrix<String>) -> ErrStr<TR> {
   let ncols = matrix.first().ok_or("No first row in matrix!")?.len();
   if let (Some(frist), _rest) = ht(cols_to_sum) {
      let mut ans: Vec<COL> = vec![colspan(frist, p("Total:"))];
      add_totals(frist, &cols_to_sum, matrix, ncols - frist, &mut ans);
      Ok(mk_tr(attrib("bgcolor", "cyan"), ans))
   } else {
      Err(format!("Cannot total given {cols_to_sum:?}"))
   }
}

fn pars(s: &String) -> Option<USD> { s.parse().ok() }

fn add_totals(last: usize, totals: &Vec<usize>, lines: &Matrix<String>,
              left: usize, acc: &mut Vec<COL>) {
   println!("Calling add_totals with {last} {totals:?} {left}");
   if let (Some(col), rest) = ht(totals) {
      let skip = col - last;
      acc.append(&mut blank_cols(skip));
      let col_with_header = column_view(&lines, col);
      let sum: USD = col_with_header.iter().filter_map(pars).sum();
      let mut summer =
         vec![TD((attrib("align","right"), p(&format!("{sum}"))))];
      acc.append(&mut summer);
      add_totals(col, &rest, lines, left - skip - 1, acc);
   } else {
      acc.append(&mut blank_cols(left));
   }
}
