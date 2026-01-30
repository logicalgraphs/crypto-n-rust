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

pub fn parse_lines<T>(f: impl Fn(String) -> ErrStr<T>, lines: &Vec<String>,
                      skip_header: Option<usize>) -> ErrStr<Vec<T>> {
   lines.into_iter()
        .skip(skip_header.unwrap_or(0))
        .map(compose!(f)(String::to_string))
        .collect()
}  

pub fn words(s: &str) -> Vec<String> {
   s.split_whitespace().map(to_string).collect()
}

pub mod functional_tests {

   use super::*;
   use std::fmt;
   use book::utils::pred;

   fn same<T:PartialEq + fmt::Display>(a: T, b: T) -> ErrStr<()> {
      pred(a == b, ()).ok_or(format!("{a} is not equal to {b}))
   }

   fn words_test() -> ErrStr<()> {
      let lorem =
         words("Lorem ipsum dolor sit amet, consectetur adipiscing elit.");
      same(8, lorem.len())
   }

   pub fn runoff() -> ErrStr<()> {
      
#[cfg(test)]
mod tests {

   use super::*;

   #[test]
   fn test_words() {
      let lorum = words("The quick, brown fox jumped over the lazy dog.");
      assert_eq!(10, lorum.len())
   }
}

