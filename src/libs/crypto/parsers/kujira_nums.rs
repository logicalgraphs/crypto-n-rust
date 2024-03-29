use book::num_utils::parse_commaless;

/*

The problem of @TeamKujira numbers.

Kujira numbers, when copypasta'd come out as a mess:

0.9362 is pasted as

0.
9362

Ugh.

This parser parses the kujira-number (above) into an f32 in the Maybe monad.

*/

pub fn parse_kujira_number(lines: &Vec<String>) -> Result<f32, String> {
   let window: Vec<&String> = lines.into_iter().take(2).collect();
   if let [win, dow] = window.as_slice() {
      if let Ok(whole) = parse_commaless(&win) {
         if let Ok(fract) = parse_commaless(&format!("0.{}", &dow)) {
            Ok(whole + fract)
         } else {
            Err(format!("Couldn't parse '{dow}' as fractional part."))
         }
      } else {
         Err(format!("Could not parse '{win}' as the whole-number part."))
      }
   } else {
      Err("Past end of file.".to_string())
   }
}

