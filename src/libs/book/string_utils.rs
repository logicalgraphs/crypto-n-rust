pub fn dequote(mut str: String) -> String {
   str.pop();
   str.remove(0);
   str
}

pub fn str_str(str: &str) -> String {
   let mut ans = String::new();
   ans.push_str(str);
   ans
}
