use book::{
   csv_utils::CsvWriter,
   html_utils::a
};

use crate::books::{Book,estimate,ticker,url};

pub struct LinkedBook { book: Book }

pub fn mk_linked(b: &Book) -> LinkedBook { LinkedBook { book: b.clone() } }

impl CsvWriter for LinkedBook {
   fn as_csv(&self) -> String {
      let book = &self.book;
      let lnk = a(&url(book), &ticker(book));
      let vol = estimate(book);
      format!("{lnk},{vol}")
   }
}
