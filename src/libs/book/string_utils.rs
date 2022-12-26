pub fn dequote(mut str: String) -> String {
   str.pop();
   str.remove(0);
   str
}

pub fn str_string(str: &str) -> String {
   let mut ans = String::new();
   ans.push_str(str);
   ans
}

pub fn plural(n: u32, noun: &str) -> String {
   let s = if n == 1 { "" } else { "s" };
   format!("{n} {noun}{s}")
}
