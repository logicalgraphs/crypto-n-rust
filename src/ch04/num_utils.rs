// a set of types and traits for weird, or decorated, numbers.

use std::fmt;

pub trait Reader {
   fn read(String) -> &self;
}

pub struct USD {
   value: f32
}

impl fmt::Display for USD {
   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
      write!("${}", s
}
