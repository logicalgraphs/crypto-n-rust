// There are two parts to parsing money-market data:

// form, and substance

// Here, we deal with the form

use std::collections::HashSet;

use crate::rows::{Row,find_triple};

use book::{
   csv_utils::print_csv,
   file_utils::extract_date_and_body,
   list_utils::{head,split},
   utils::get_args
};

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

fn usage() {
   println!("\n./data_entry <file>");
   println!("\n\tConvert file of quotes and amounts to just amounts.\n");
}

pub fn process() {
   process1(|elt| elt.clone());
}

pub fn process1(preformatter: impl Fn(&Vec<String>) -> Vec<String>) {
   if let Some(file) = head(get_args()) {
      let (date, body) = extract_date_and_body(&file);
      let new_bod = preformatter(&body);
      split(new_bod, "Borrowed".to_string()).iter()
          .fold(1.0, preprocess_with_sign(&date));
   } else {
      usage();
   }
}

// a function that 'folds over' the (negative) sign
fn preprocess_with_sign(date: &str) -> impl Fn(f32, &Vec<String>) -> f32 + '_ {
   |sign: f32, lines: &Vec<String>| {
      let lines1: Vec<&String> =
         lines.iter().filter(|x| !x.is_empty()).collect();
      let mut assets: HashSet<Row> = HashSet::new();
      then_process(date, sign, &lines1, &mut assets);
      assets.iter().for_each(print_csv);
      sign * -1.0
   }
}

fn then_process(date: &str, sign: f32, lines: &Vec<&String>, 
                assets: &mut HashSet<Row>) {
   if let Some((idx, row)) = find_triple(lines) {
      println!("{}", sign * row.amount);
      assets.insert(row);
      let (_, new_lines) = lines.split_at(idx+3);
      then_process(date, sign, &new_lines.to_vec(), assets);
   }
}
