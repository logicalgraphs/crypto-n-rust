use crate::{
   compose,
   err_utils::ErrStr
};

pub fn dequote(str: &String) -> String {
   str.strip_prefix("\"")
      .and_then(|s1| s1.strip_suffix("\""))
      .unwrap_or(str)
      .to_string()
}

pub fn bracket(brackets: &str, body: &str) -> String {
   let mut i = brackets.chars();
   let frist = i.next().unwrap();
   let last = i.last().unwrap();
   format!("{frist}{body}{last}")
}

pub fn quot(s: &str) -> String {
   bracket("\"\"", s)
}

pub fn plural(n: usize, noun: &str) -> String {
   let s = if n == 1 { "" } else { "s" };
   format!("{n} {noun}{s}")
}

pub fn to_string(s: &str) -> String { s.to_string() }
pub fn s(st: &str) -> String { to_string(st) } // shorthand for to_string()

pub fn parse_lines<T>(f: impl Fn(String) -> ErrStr<T>, lines: &Vec<String>,
                      skip_header: Option<usize>) -> ErrStr<Vec<T>> {
   lines.into_iter()
        .skip(skip_header.unwrap_or(0))
        .map(compose!(f)(String::to_string))
        .collect()
}  

pub fn str2strf<T>(f: impl Fn(&str) -> T) -> impl Fn(String) -> T {
   move | s: String | f(&s)
}

pub fn words(st: &str) -> Vec<String> {
   st.split_whitespace().map(s).collect()
}

pub fn lines(st: &str) -> Vec<String> {
   st.split("\n").map(s).collect()
}

// ----- TESTS -------------------------------------------------------

#[cfg(test)]
#[cfg(not(tarpaulin_include))]
mod poem_san {
   pub fn ee_cummings() -> String {
"since feeling is first
who pays any attention 
to the syntax of things
will never wholly kiss you;

wholly to be a fool
while Spring is in the world

my blood approves,
and kisses are a better fate
than wisdom
lady i swear by all flowers. Don’t cry
—the best gesture of my brain is less than
your eyelids’ flutter which says

we are for each other: then
laugh, leaning back in my arms
for life’s not a paragraph

And death i think is no parenthesis".to_string()
   }
}

#[cfg(test)]
#[cfg(not(tarpaulin_include))]
pub mod functional_tests {

   use super::*;
   use super::poem_san::ee_cummings;
   use paste::paste;
   use crate::{ create_testing, compose, utils::debug };

   create_testing!("string_utils");
   run_with!("words",
        "Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
        compose!(debug)(words));
   run_with!("lines", &ee_cummings(), compose!(debug)(lines));
}

#[cfg(test)]
#[cfg(not(tarpaulin_include))]
mod tests {

   use super::*;
   use super::poem_san::ee_cummings;

   #[test] fn test_words() {
      let lorum = words("The quick, brown fox jumped over the lazy dog.");
      assert_eq!(9, lorum.len())
   }

   #[test] fn test_lines() {
      let poem = lines(&ee_cummings());
      assert_eq!(20, poem.len());
   }

   #[test] fn test_singular() {
      let apple = plural(1, "apple");
      assert_eq!("1 apple", &apple);
   }

   #[test] fn test_plural() {
      let kumquats = plural(2, "kumquat");
      assert_eq!("2 kumquats", &kumquats);
   }
}

