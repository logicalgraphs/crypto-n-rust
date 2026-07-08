use std::{ collections::HashSet, fmt, ops::Deref };

use crate::{ compose, err_utils::ErrStr };

pub fn to_string(s: &str) -> String { s.to_string() }
pub fn s(st: &str) -> String { to_string(st) } // shorthand for to_string()

pub fn str2strf<T>(f: impl Fn(&str) -> T) -> impl Fn(String) -> T {
   move | s: String | f(&s)
}

pub fn words(st: &str) -> Vec<String> {
   st.split_whitespace().map(s).collect()
}

pub fn lines(st: &str) -> Vec<String> {
   st.split("\n").map(s).collect()
}

pub fn plural(n: usize, noun: &str) -> String {
   let s = if n == 1 { "" } else { "s" };
   format!("{n} {noun}{s}")
}

// ----- Article --------------------------------------------------------------
// added by bparis to pivoteur protocol, moved here by dma

pub fn article(word: &str) -> ErrStr<String> {
   let vowels: HashSet<char> = "AEIOU".chars().collect();
   Ok(format!("a{} {word}",
           if vowels.contains(&word.chars()
                                   .next()
                                   .ok_or("empty string for article")?
                                   .to_ascii_uppercase()) { "n" } else { "" }))
}

// ----- UppercaseString --------------------------------------------------

/// A string wrapper that guarantees all alphabetic characters are uppercase.
#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct UppercaseString(String);

impl UppercaseString {
   /// Creates a new UppercaseString, converting all input characters 
   /// to uppercase.
   pub fn new(val: &str) -> Self { Self(val.to_uppercase()) }
}

// 1. Allow seamless conversion from standard strings
impl From<&str> for UppercaseString {
   fn from(val: &str) -> Self { Self::new(val) }
}

impl From<String> for UppercaseString {
   fn from(val: String) -> Self {
      // Optimizes by modifying the allocation in-place if possible
      let mut s = val;
      s.make_ascii_uppercase();
      // Note: use .to_uppercase() if handling full Unicode casing
      Self(s)
   }
}

// 2. Allow treating it like a standard string slice (&str) via Deref
impl Deref for UppercaseString {
   type Target = str;

   fn deref(&self) -> &Self::Target { &self.0 }
}

// 3. Enable printing via println!("{}", upper_str);
impl fmt::Display for UppercaseString {
   fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      write!(f, "{}", self.0)
   }
}

// ----- Parsing ------------------------------------------------------------

pub fn parse_lines<T>(f: impl Fn(String) -> ErrStr<T>, lines: &Vec<String>,
                      skip_header: Option<usize>) -> ErrStr<Vec<T>> {
   lines.into_iter()
        .skip(skip_header.unwrap_or(0))
        .map(compose!(f)(String::to_string))
        .collect()
}  

// ----- JSON functions ----------------------------------------------------

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

pub fn quot(s: &str) -> String { bracket("\"\"", s) }

// ----- TESTS -------------------------------------------------------

#[cfg(test)]
#[cfg(not(tarpaulin_include))]
mod poem_san {
   use super::s;
   pub fn ee_cummings() -> String {
s("since feeling is first
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

And death i think is no parenthesis")
   }

   pub fn greet() -> &'static str { "hello, world!\n123" }
}

#[cfg(test)]
#[cfg(not(tarpaulin_include))]
mod functional_tests {

   use super::*;
   use super::poem_san::{ ee_cummings, greet };
   use paste::paste;
   use crate::{ create_testing, compose, utils::debug };

   create_testing!("string_utils");
   run_with!("words",
        "Lorem ipsum dolor sit amet, consectetur adipiscing elit.",
        compose!(debug)(words));
   run_with!("lines", &ee_cummings(), compose!(debug)(lines));
   run_with!("new", " (UppercaseString)", greet(), UppercaseString::new);
}

#[cfg(test)]
#[cfg(not(tarpaulin_include))]
mod tests {

   use super::*;
   use super::poem_san::{ ee_cummings, greet };

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

   #[test] fn test_article_an_for_vowel() -> ErrStr<()> {
      let msg = article("AVAX-on-BTC pivot")?;
      assert_eq!("an AVAX-on-BTC pivot", &msg);
      Ok(())
   }

   #[test] fn test_article_a_for_consonant() -> ErrStr<()> {
      let msg = article("BTC-on-ETH pivot")?;
      assert_eq!("a BTC-on-ETH pivot", &msg);
      Ok(())
   }

   #[test] fn test_uppercase_string_len() {
      let greeting = greet();
      let hello = UppercaseString::new(greet());
      assert_eq!(greeting.len(), hello.len());
   }

   #[test] fn test_uppercase_string_contains() {
      let greeting = greet();
      let hello = UppercaseString::new(greet());
      assert!(greeting.contains("ell"));
      assert!(hello.contains("ELL"));
      assert!(!hello.contains("ell"));
   }
}

