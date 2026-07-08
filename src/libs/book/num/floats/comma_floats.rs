use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct CommaFloat(pub f32);

// Enable reading/parsing from a string containing commas
impl FromStr for CommaFloat {
    type Err = std::num::ParseFloatError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // Remove all commas from the string before parsing
        let cleaned = s.replace(',', "");
        let parsed = cleaned.parse::<f32>()?;
        Ok(CommaFloat(parsed))
    }
}

// Enable standard f32 display without commas
impl fmt::Display for CommaFloat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<CommaFloat> for f32 {
   fn from(comma_float: CommaFloat) -> f32 { comma_float.0 }
}

// ----- TESTS -------------------------------------------------------

#[cfg(test)]
#[cfg(not(tarpaulin_include))]
mod functional_tests {
   use super::*;
   use paste::paste;
   use crate::{
      create_testing,
      compose,
      err_utils::{ ErrStr, err_or },
      utils::resolve
   };

   create_testing!("num::floats::comma_floats");

   fn parse_commaed(s: &str) -> ErrStr<CommaFloat> {
      err_or(s.parse(), &format!("Unable to parse float {s}"))
   }
   run_with!("parse", "1,234,567.90", compose!(resolve)(parse_commaed));
}
