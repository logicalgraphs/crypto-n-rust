use book::{
   csv_utils::CsvWriter
};

use crypto::types::percentage::{Percentage,mk_percentage};

use crate::books::{Book,ticker};

pub struct BookShare { book: Book, perc: Percentage }

fn to_book_share(tot: f32, book: &Book) -> BookShare {
   let my_vol = book.vol_24h;
   BookShare { book: book.clone(), perc: mk_percentage(my_vol / tot) }
}

pub fn analyze(books: &Vec<Book>) -> Vec<BookShare> {
   let total = books.into_iter().map(|b| b.vol_24h).sum::<f32>();
   books.into_iter().map(|b| to_book_share(total, b)).collect()
}

impl CsvWriter for BookShare {
   fn as_csv(&self) -> String {
      let b = &self.book;
      format!("{},{},{}", ticker(b), b.vol_24h, &self.perc)
   }
}
