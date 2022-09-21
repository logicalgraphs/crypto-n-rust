pub fn dequote(mut str: String) -> String {
   str.pop();
   str.remove(0);
   str
}
