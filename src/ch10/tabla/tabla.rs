use book::{
   err_utils::{ErrStr,err_or},
   html_utils::{AsHTML,mk_table,attrib,TR,colspan,blank_cols,
                COL,p,COL::TD,mk_tr},
   list_utils::ht,
   matrix_utils::{Matrix,column_view},
   string_utils::to_string,
   stream_utils::lines_from_stream,
   utils::{get_args,pred}
};

use crypto::types::usd::USD;

fn main() -> ErrStr<()> {
   let args = get_args();
   if let Some(help) = args.first() {
      if help == "--help" { return usage() }
   }
   restage(args)
}

fn restage(args: Vec<String>) -> ErrStr<()> {
   let lines: Matrix<String> =
      lines_from_stream().into_iter()
                         .map(|l| l.split("\t").map(to_string).collect())
                         .collect();
   deuceage(&lines, args)
}

fn usage() -> ErrStr<()> {
   println!("./tabla: enHTMLTableifies rows of data.\n");
   println!("usage:\n");
   println!("$ echo <TSV data> | ./tabla [col1 [col2] [col3...]]\n");
   println!("Spits out an HTML table of the <TSV data> with an optional");
   println!("footer row of the totals, columns zero-indexed-based.");
   Ok(())
}

fn deuceage(lines: &Matrix<String>, cols: Vec<String>) -> ErrStr<()> {
   fn n(c: &String) -> Option<usize> {
      err_or(c.parse(), &format!("Cannot parse number {c}")).ok()
   }
   let mut to_total: Vec<usize> = cols.iter().filter_map(n).collect();
   let footer = pred(!to_total.is_empty(), {
      to_total.sort();
      totals_of(&to_total, lines)?
   });
   println!("{}", mk_table(&lines, footer).as_html());
   Ok(())
}

fn totals_of(cols_to_sum: &Vec<usize>, matrix: &Matrix<String>) -> ErrStr<TR> {
   let ncols = matrix.first().ok_or("No first row in matrix!")?.len();
   if let (Some(frist), _rest) = ht(cols_to_sum) {
      let mut ans: Vec<COL> = vec![colspan(frist, p("Total:"))];
      add_totals(frist - 1, &cols_to_sum, matrix, ncols - frist, &mut ans);
      Ok(mk_tr(attrib("bgcolor", "cyan"), ans))
   } else {
      Err(format!("Cannot total given {cols_to_sum:?}"))
   }
}

fn pars(s: &String) -> Option<USD> { s.parse().ok() }

fn add_totals(last: usize, totals: &Vec<usize>, lines: &Matrix<String>,
              left: usize, acc: &mut Vec<COL>) {
   if let (Some(col), rest) = ht(totals) {
      let skip = col - last - 1;
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
