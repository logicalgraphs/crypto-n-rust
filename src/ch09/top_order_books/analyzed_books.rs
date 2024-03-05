use book::csv_utils::CsvWriter;

use crypto::types::{
   books::{Book,ticker,vol_24h},
   percentage::{Percentage,mk_percentage}
};

pub struct BookShare { book: Book, perc: Percentage }

fn to_book_share(tot: f32) -> impl Fn(&Book) -> BookShare {
   move |book: &Book| {
      let my_vol = vol_24h(book).amount;
      BookShare { book: book.clone(), perc: mk_percentage(my_vol / tot) }
   }
}

pub fn analyze(books: &Vec<Book>) -> Vec<BookShare> {
   let total = books.into_iter().map(|b| vol_24h(b).amount).sum::<f32>();
   books.into_iter().map(to_book_share(total)).collect()
}

impl CsvWriter for BookShare {
   fn as_csv(&self) -> String {
      let b = &self.book;
      format!("{},{},{}", ticker(b), vol_24h(b), &self.perc)
   }
   fn ncols(&self) -> usize { 3 }
}
