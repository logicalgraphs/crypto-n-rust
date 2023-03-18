// represents (truncated) percentages

use std::{fmt,cmp::Ordering};

use crypto::types::percentage::{Percentage,mk_percentage};

#[derive(Debug, Clone)]
pub struct Perc { p: Percentage }

pub fn mk_perc(p: f32) -> Perc {
   Perc { p: mk_percentage(p) }
}

impl fmt::Display for Perc {
   fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
      let p: i32 = (self.p.percent * 100.0) as i32;
      write!(formatter, "{p}%")
   }        
}

impl PartialEq for Perc {
   fn eq(&self, other: &Self) -> bool {
      self.p == other.p
   }
}

impl Eq for Perc { }

impl PartialOrd for Perc {
   fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
      self.p.partial_cmp(&other.p)
   }
}

