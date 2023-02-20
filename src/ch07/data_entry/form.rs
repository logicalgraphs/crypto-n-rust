// There are two parts to parsing money-market data:

// form, and substance

// Here, we deal with the form

use book::{
   list_utils::{head},
   num_utils::parse_commaless
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

But first thing to do is to get to a starting point:

Old code working with the form/substance bifurcation.
*/

// a function that 'folds over' the (negative) sign
pub fn preprocess_with_sign(sign: f32, lines: &mut Vec<String>) -> f32 {
   let mut lines1: Vec<&String> = lines.iter().filter(|x| !x.is_empty()).collect();
   lines1.retain(|s| two(s));
   for line in lines1 {
      if let Some(position) = head(line.split(' ').collect()) {
         let num: f32 = parse_commaless(&position.to_string())
                                 .expect("Not a number");
         println!("{}", sign * num);
      }
   }
   sign * -1.0
}

fn two(line: &String) -> bool {
   let words: Vec<&str> = line.split(' ').collect();
   words.len() > 1 && head(words) != Some("Balance:")
}
