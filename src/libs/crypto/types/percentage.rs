// types, regardless underlying data sources

use std::{
   fmt,
   str::FromStr
};

#[derive(Debug, Clone)]
pub struct Percentage {
   pub percent: f32
}

// ----- implementations -----------------------------------------------------

impl PartialEq for Percentage {
   fn eq(&self, other: &Self) -> bool {
      self.percent == other.percent
   }
}

impl Eq for Percentage { }

impl fmt::Display for Percentage {
   fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
      write!(formatter, "{:.2}%", self.percent)
   }
}

impl FromStr for Percentage {
   type Err = String;
   fn from_str(elt: &str) -> Result<Self, String> {
      let mut per = elt.to_string();
      per.pop();
      let percent = per.parse();
      if let Ok(percent_p) = percent {
         Ok(mk_percentage(percent_p))
      } else {
         Err(format!("Not a percentage: {elt}"))
      }
   }
}

// ----- ... and our methods -------------------------------------------------

pub fn mk_percentage(percent: f32) -> Percentage {
   Percentage { percent }
}
