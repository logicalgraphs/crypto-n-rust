use book::{
   csv_utils::CsvWriter,
   html_utils::{a,Mode,roff}
};

use crypto::types::interfaces::{Book,estimate,ticker,url};

pub struct LinkedBook { book: Book }

pub fn mk_linked(b: &Book) -> LinkedBook { LinkedBook { book: b.clone() } }

impl CsvWriter for LinkedBook {
   fn as_csv(&self) -> String {
      let book = &self.book;
      let lnk = roff(&Mode::HTML, &a(&url(book), &ticker(book)));
      let vol = estimate(book);
      format!("{lnk},{vol}")
   }
   fn ncols(&self) -> usize { 2 }
}
