// just the every day HTMLification-stupfen

use crate::string_utils::quot;

pub fn h(n: u8, content: &str) -> String {
   elt(&format!("h{n}"), content)
}

pub fn ol(list: &Vec<String>) -> String {
   let lis: Vec<String> = list.iter().map(|itm| li(itm)).collect();
   elt("ol", &lis.join("\n"))
}

pub fn a(url: &str, content: &str) -> String {
   let a = vec![("href", url)];
   eattrs("a", &a, content)
}

pub fn p(content: &str) -> String {
   elt("p", content)
}

// ----- Helper functions ---------------------------------------------

fn li(s: &str) -> String {
  elt("li", s)
}

fn elt(tag: &str, content: &str) -> String {
   format!("<{tag}>{content}</{tag}>")
}

fn eattrs(tag: &str, attribs: &Vec<(&str, &str)>, content: &str) -> String {
   format!("<{tag} {}>{content}</{tag}>", attrs(attribs))
}

fn attrs(attribs: &Vec<(&str, &str)>) -> String {
   let new_attribs: Vec<String> = attribs.iter().map(|p| attr(p)).collect();
   new_attribs.join(" ")
}

fn attr(a: &(&str, &str)) -> String {
   format!("{}={}", quot(a.0), quot(a.1))
}
