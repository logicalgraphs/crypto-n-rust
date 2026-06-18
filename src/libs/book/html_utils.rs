// just the every day HTMLification-stupfen

use strum_macros::EnumIter; // 0.17.1

use crate::{
   list_utils::ht,
   matrix_utils::Matrix,
   num_utils::parse_commaless,
   string_utils::{quot,s}
};

#[derive(Debug,Clone)]
pub enum HTML {
   BODY(Vec<HTML>),
   TABLE(Vec<TR>),
   H((usize,String)),
   OL(Vec<LI>),
   A((String, String)),
   P(String),
   CODE(String),
   NBSP
}
use HTML::*;

#[derive(Debug,Clone)]
pub struct LI { line: String }

pub fn mk_li(l: &str) -> LI { LI { line: s(l) } }

// ----- TABLES ------------------------------------------------------

type Attrib = (String, String);

#[derive(Debug,Clone)]
pub struct TR { attribs: Vec<Attrib>, row: Vec<COL> }

pub fn mk_tr(attribs: Vec<Attrib>, row: Vec<COL>) -> TR { TR { attribs, row } }

#[derive(Debug,Clone)]
pub enum COL {
   TD((Vec<Attrib>, HTML)),
   TH(HTML)     // TH((Vec<Attrib>, HTML)),
}
use COL::*;

pub fn mk_table(matrix: &Matrix<String>, footer: Option<TR>) -> HTML {
   let (some_header, rows) = ht(matrix);
   let header = some_header.unwrap();
   let ncols = header.len();
   fn th(s: &String) -> COL { TH(p(s)) }
   let head = header.iter().map(th).collect();
   fn align_num(s: &str) -> Vec<Attrib> {
      attrib("align", s.strip_prefix("$").or(Some(s)).and_then(|s1| {
         let n: Result<f32, _> = parse_commaless(s1);
         Some(match n { Ok(_) => "right", _ => "left" })
      }).unwrap())
   }
   fn td(s: &str) -> COL { TD((align_num(s), p(s))) }
   fn tr(v: &[String]) -> TR {
      TR { attribs: vec![], row: v.iter().map(td).collect() }
   }
   let mut ans = Vec::new();
   ans.push(TR { attribs: attrib("bgcolor", "cyan"), row: head });
   let mut trs: Vec<TR> = rows.iter().map(tr).collect();
   ans.append(&mut trs);
   if let Some(foot) = footer {
      let mut rows = vec![blank_row(ncols), foot];
      ans.append(&mut rows);
   }
   TABLE(ans)
}

pub fn no_attribs() -> Vec<Attrib> { Vec::new() }

fn blank_row(cols: usize) -> TR {
   TR { attribs: no_attribs(), row: vec![colspan(cols, nbsp())] }
}

pub fn blank_cols(n: usize) -> Vec<COL> {
   let mut cols = Vec::new();
   for _i in 0 .. n { cols.push(blank_col()) }
   cols
}

fn blank_col() -> COL { TD((no_attribs(), nbsp())) }

pub fn colspan(cols: usize, content: HTML) -> COL {
   TD ((attrib("colspan", &format!("{cols}")), content))
}

// ----- MODES -------------------------------------------------------

#[derive(PartialEq, EnumIter)]
pub enum Mode { HTML, TEXT, CSV }

pub fn mk_mode(m: &str) -> Mode {
   match m.to_lowercase().as_str() {
      "html" => Mode::HTML,
      "text" => Mode::TEXT,
      _      => panic!("Do not know the mode {}", m)
   }
}

// ----- HTML mode -------------------------------------------------------

pub trait AsHTML {
   fn as_html(&self) -> String;
}

impl AsHTML for LI {
   fn as_html(&self) -> String { elt("li", &self.line) }
}

impl AsHTML for COL {
   fn as_html(&self) -> String {
      match &self {
         COL::TH(html) => elt("th", &html.as_html()),
         COL::TD((attrs, html)) => eattrs("td", &attrs, &html.as_html())
      }
   }
}

impl AsHTML for TR {
   fn as_html(&self) -> String {
      eattrs("tr", &self.attribs, &list_h(&self.row))
   }
}

impl AsHTML for HTML {
   fn as_html(&self) -> String {
      match &self {
         BODY(elts) => list_h(&elts),
         TABLE(rows) =>
            eattrs("table",
                   &attribs(&[("border", "1"),("width","75%"),
                              ("align", "center")]),
                   &list_h(&rows)),
         H((n, title)) =>
            format!("{}\n{}", elt(&format!("h{n}"), title), nbsp().as_html()),
         OL(lis) => elt("ol", &list_h(&lis)),
         A((url, content)) => eattrs("a", &attrib("href", url), &content),
         P(content) => elt("p", content),
         CODE(code) => elt("code", code),
         NBSP => elt("p", "&nbsp;")
      }
   }
}

fn list_h<T: AsHTML>(v: &Vec<T>) -> String {
   let v1: Vec<String> = v.iter().map(|e| e.as_html()).collect();
   v1.join("\n")
}

// ----- TEXT mode -------------------------------------------------------

pub trait AsText {
   fn as_text(&self) -> String;
}

impl AsText for LI {
   fn as_text(&self) -> String { self.line.to_string() }
}

impl AsText for COL {
   fn as_text(&self) -> String {
      match &self {
         TH(html) => elt("th", &html.as_text()),
         TD((_, html)) => elt("td", &html.as_text())
      }
   }
}

impl AsText for TR {
   fn as_text(&self) -> String {
      let r: Vec<String> = self.row.iter().map(AsText::as_text).collect();
      r.join("\t")
   }
}

impl AsText for HTML {
   fn as_text(&self) -> String {
      match &self {
         BODY(elts) => { list_t(elts, false) },
         TABLE(rows) => { list_t(rows, false) },
         H((_, title)) => div(&title),
         OL(lis) => list_t(&lis, true),
         A((url, content)) => format!("{content} ({url})"),
         P(content) => div(&content),
         CODE(code) => code.to_string(),
         NBSP => "".to_string()
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

// ----- CSV mode -------------------------------------------------------

pub trait AsCSV {
   fn as_csv(&self) -> String;
}

impl AsCSV for HTML {
   fn as_csv(&self) -> String { "".to_string() } // eheh: not much here, eh?
}

// ----- Run-off functions --------------------------------------------------

pub fn roff(mode: &Mode, content: &HTML) -> String {
   match mode {
      Mode::HTML => content.as_html(),
      Mode::TEXT => content.as_text(),
      Mode::CSV  => content.as_csv()
   }
}

pub fn proff(elt: &HTML, mode: &Mode) {
   println!("{}", roff(mode, elt));
}

// ----- HTML-constructors --------------------------------------------------

pub fn body(content: &[HTML]) -> HTML { BODY(content.to_vec()) }
pub fn h(n: usize, titl: &str) -> HTML { H((n, s(titl))) }

pub fn ol(list: &[String]) -> HTML {
   let lis: Vec<LI> = list.iter().map(mk_li).collect();
   OL(lis)
}

pub fn a(url: &str, content: &str) -> HTML { A((s(url), s(content))) }
pub fn p(content: &str) -> HTML { P(s(content)) }
pub fn nbsp() -> HTML { NBSP }

pub fn attrib(name: &str, value: &str) -> Vec<Attrib> {
   vec![mk_attrib(&(name, value))]
}

pub fn mk_attrib(attr: &(&str, &str)) -> Attrib { (s(attr.0), s(attr.1)) }

pub fn attribs(attrs: &[(&str, &str)]) -> Vec<Attrib> {
   attrs.iter().map(mk_attrib).collect()
}

// ----- Helper functions ---------------------------------------------

fn elt(tag: &str, content: &str) -> String {
   format!("<{tag}>{content}</{tag}>")
}

fn eattrs(tag: &str, attribs: &Vec<Attrib>, content: &str) -> String {
   format!("<{tag} {}>{content}</{tag}>", attrs(attribs))
}

fn attrs(attribs: &Vec<Attrib>) -> String {
   let new_attribs: Vec<String> = attribs.iter().map(attr).collect();
   new_attribs.join(" ")
}

fn attr(a: &Attrib) -> String {
   format!("{}={}", a.0, &quot(&a.1))
}

// ----- TESTS -------------------------------------------------------

#[cfg(test)]
#[cfg(not(tarpaulin_include))]
mod functional_tests {
   use super::*; 
   use paste::paste;
   use crate::{create_testing, string_utils::{lines, words}};

   create_testing!("html_utils");

   fn dom() -> HTML {
      body(
         &vec![h(1, "Welcome to My Sample HTML Webpage"),
               p("This is a standard paragraph element used to display text. 
It provides a simple way to organize sentences and build core readable content 
on a webpage."),
               p("You can find more educational coding resources by visiting 
the official"),
               a("https://www.w3schools.com", "W3Schools website"),
               h(2, "Common Web Development Languages"),
            /* ol(lines("HTML (HyperText Markup Language)
CSS (Cascading Style Sheets)
JavaScript")),  */
               h(2, "Steps to Launch a Website"),
               ol(lines("Write the code using an editor.
Test the document in a web browser.
Upload the files to a hosting server.")),
               h(2, "Project team roles"),
               mk_table(vec![words("Name Role"),
                 vec!["Alice Cooper", "Frontend developer"],
                 vec!["Bob Plant", "UI/UX designer"]], None)])
   }
}
