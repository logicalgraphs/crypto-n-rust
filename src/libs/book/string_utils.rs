use crate::compose;

pub fn dequote(mut str: String) -> String {
   str.pop();
   str.remove(0);
   str
}

pub fn quot(s: &str) -> String {
   format!("\"{s}\"")
}

pub fn plural(n: usize, noun: &str) -> String {
   let s = if n == 1 { "" } else { "s" };
   format!("{n} {noun}{s}")
}

pub fn to_string(s: &str) -> String { s.to_string() }

pub fn parse_lines<T>(f: impl Fn(String) -> Result<T, String>,
                      lines: &Vec<String>, skip_header: Option<usize>)
    -> Result<Vec<T>, String> {
   lines.into_iter()
        .skip(skip_header.unwrap_or(0))
        .map(compose!(f)(String::to_string))
        .collect()
}  
