// just the every day HTMLification-stupfen

use strum_macros::EnumIter; // 0.17.1

use crate::string_utils::quot;

#[derive(Debug,Clone)]
pub enum HTML {
   BODY(Vec<HTML>),
   H((usize,String)),
   OL(Vec<LI>),
   A((String, String)),
   P(String)
}

#[derive(Debug,Clone)]
pub struct LI { line: String }

fn mk_li(l: &String) -> LI {
   LI { line: l.to_string() }
}

// ----- MODES -------------------------------------------------------

#[derive(PartialEq, EnumIter)]
pub enum Mode { HTML, TEXT }

pub fn mk_mode(m: &str) -> Mode {
   match m.to_lowercase().as_str() {
      "html" => Mode::HTML,
      "text" => Mode::TEXT,
      _      => panic!("Do not know the mode {}", m)
   }
}

pub trait AsHTML {
   fn as_html(&self) -> String;
}

impl AsHTML for LI {
   fn as_html(&self) -> String { elt("li", &self.line) }
}

impl AsHTML for HTML {
   fn as_html(&self) -> String {
      match &self {
         HTML::BODY(elts) => { list_h(&elts) },
         HTML::H((n, title)) => elt(&format!("h{n}"), title),
         HTML::OL(lis) => elt("ol", &list_h(&lis)),
         HTML::A((url, content)) => {
            let a = vec![("href", url)];
            eattrs("a", &a, &content)
         },
         HTML::P(content) => elt("p", content)
      }
   }
}

fn list_h<T: AsHTML>(v: &Vec<T>) -> String {
   let v1: Vec<String> = v.iter().map(|e| e.as_html()).collect();
   v1.join("\n")
}

pub trait AsText {
   fn as_text(&self) -> String;
}

impl AsText for LI {
   fn as_text(&self) -> String { self.line.to_string() }
}

impl AsText for HTML {
   fn as_text(&self) -> String {
      match &self {
         HTML::BODY(elts) => { list_t(elts, false) },
         HTML::H((_, title)) => div(&title),
         HTML::OL(lis) => list_t(&lis, true),
         HTML::A((url, content)) => format!("{content} ({url})"),
         HTML::P(content) => div(&content)
      }
   }
}

fn div(content: &str) -> String { format!("\n{content}\n") }

fn list_t<T: AsText>(v: &Vec<T>, numerate: bool) -> String {
   let v1: Vec<String> = v.iter().enumerate().map(|(x,e)| {
      let i = format!("{}. ", x + 1);
      let idx = if numerate { &i } else { "" };
      format!("{}{}", idx, e.as_text())
   }).collect();
   v1.join("\n")
}

/*
pub fn exporter<T: AsText + AsHTML>(mode: &Mode)
   -> impl Fn(T) -> String + '_ {
   move |elt| {
      match mode {
         Mode::TEXT => elt.as_text(),
         Mode::HTML => elt.as_html()
      }
   }
}
*/

pub fn roff<T: AsText + AsHTML>(elt: &T, mode: &Mode) -> String {
   let runoft = if mode == &Mode::HTML {
      elt.as_html()
   } else {
      elt.as_text()
   };
   runoft
}

// ----- HTML-constructors --------------------------------------------------

pub fn body(content: &Vec<HTML>) -> HTML {
   HTML::BODY(content.to_vec())
}

pub fn h(n: usize, titl: &str) -> HTML {
   HTML::H((n, titl.to_string()))
}

pub fn ol(list: &Vec<String>) -> HTML {
   let lis: Vec<LI> = list.iter().map(mk_li).collect();
   HTML::OL(lis)
}

pub fn a(url: &str, content: &str) -> HTML {
   HTML::A((url.to_string(), content.to_string()))
}

pub fn p(content: &str) -> HTML {
   HTML::P(content.to_string())
}

pub fn nbsp() -> HTML {
   HTML::P("&nbsp;".to_string())
}

// ----- Helper functions ---------------------------------------------

/*
fn li(s: &String) -> String {
  elt("li", s)
}
*/

fn elt(tag: &str, content: &str) -> String {
   format!("<{tag}>{content}</{tag}>")
}

fn eattrs(tag: &str, attribs: &Vec<(&str, &String)>, content: &str) -> String {
   format!("<{tag} {}>{content}</{tag}>", attrs(attribs))
}

fn attrs(attribs: &Vec<(&str, &String)>) -> String {
   let new_attribs: Vec<String> = attribs.iter().map(attr).collect();
   new_attribs.join(" ")
}

fn attr(a: &(&str, &String)) -> String {
   format!("{}={}", a.0, quot(a.1))
}
