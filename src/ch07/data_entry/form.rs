// There are two parts to parsing money-market data:

// form, and substance

// Here, we deal with the form

use std::collections::HashSet;

use crate::rows::{Row,find_triple};

use book::csv_utils::print_csv;

// And here is the form:

/*
Money-market files are split into two: supply-side and borrow-side

The delimiter is "Balance" (in the two cases I've dealt with)

The output is positive and negative values

We also have (something like):

token (name)
[maybe junque]
[maybe blank]
#tokens [trailing garbage]
[maybe blank]
$value

We can get rid of the blanks lines with a preprocessor.

How do we read the token name? A peek to a number?

There is this interesting read here about windows:

https://stackoverflow.com/questions/62186871/how-to-correctly-use-peek-in-rust
*/

// a function that 'folds over' the (negative) sign
pub fn preprocess_with_sign(sign: f32, lines: &mut Vec<String>) -> f32 {
   let lines1: Vec<&String> = lines.iter().filter(|x| !x.is_empty()).collect();
   let mut assets: HashSet<Row> = HashSet::new();
   process(sign, &lines1, &mut assets);
   assets.iter().for_each(print_csv);
   sign * -1.0
}

fn process(sign: f32, lines: &Vec<&String>, assets: &mut HashSet<Row>) {
   if let Some((idx, row)) = find_triple(lines) {
      println!("{}", sign * row.amount);
      assets.insert(row);
      let (_, new_lines) = lines.split_at(idx+3);
      process(sign, &new_lines.to_vec(), assets);
   }
}
