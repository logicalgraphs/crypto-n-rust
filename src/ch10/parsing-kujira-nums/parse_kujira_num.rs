use book::num_utils::parse_commaless;

pub fn parse_kujira_number(lines: &Vec<String>) -> Option<f32> {
   if let [win, dow] = lines.into_iter().take(2).collect::<Vec<_>>().as_slice() {
      if let Ok(whole) = parse_commaless(&win) {
         if let Ok(fract) = parse_commaless(&format!("0.{}", &dow)) {
            return Some(whole + fract);
         }
      }
   }
   None
}
