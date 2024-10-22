// types, regardless underlying data sources

use std::{
   cmp::Ordering,
   fmt,
   str::FromStr
};

use crate::utils::err_utils::ErrStr;

#[derive(Debug, Clone, Default)]
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
      write!(formatter, "{:.2}%", self.percent * 100.0)
   }
}

impl FromStr for Percentage {
   type Err = String;
   fn from_str(elt: &str) -> ErrStr<Self> {
      let mut per = elt.to_string();
      let perc_sym = per.pop();
      if Some('%') == perc_sym {
         let percent: Result<f32, _> = per.parse();
         match percent {
            Ok(percent_p) => Ok(mk_percentage(percent_p / 100.0)),
            Err(err) =>
               Err(format!("Could not parse percentage from {elt}: {err}"))
         }
      } else {
         Err(format!("Percentage missing terminating '%' in {elt}"))
      }
   }
}

impl PartialOrd for Percentage {
   fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
      self.percent.partial_cmp(&other.percent)
   }
}

// ----- ... and our methods -------------------------------------------------

pub fn mk_percentage(percent: f32) -> Percentage {
   Percentage { percent }
}

impl Percentage {
   pub fn of(&self, magnitude: f32) -> f32 {
      self.percent * magnitude
   }
}
