pub fn dequote(mut str: String) -> String {
   str.pop();
   str.remove(0);
   str
}

pub fn quot(s: &str) -> String {
   format!("\"{s}\"")
}

pub fn plural(n: u32, noun: &str) -> String {
   let s = if n == 1 { "" } else { "s" };
   format!("{n} {noun}{s}")
}

pub fn to_string(s: &str) -> String { s.to_string() }
