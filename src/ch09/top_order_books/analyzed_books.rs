use book::{
   csv_utils::CsvWriter,
   utils::pred
};

use crypto::types::{
   books::{Book,ticker,vol_24h},
   percentage::{Percentage,mk_percentage}
};

#[derive(Debug,Clone)]
pub struct BookShare { book: Book, perc: Percentage }

fn to_book_share(tot: f32) -> impl Fn(&Book) -> BookShare {
   move |book: &Book| {
      let my_vol = vol_24h(book).amount;
      BookShare { book: book.clone(), perc: mk_percentage(my_vol / tot) }
   }
}

pub fn analyze(books: &Vec<Book>) -> Vec<BookShare> {
   let total = books.into_iter().map(|b| vol_24h(b).amount).sum::<f32>();
   fn dust(bs: BookShare) -> Option<BookShare> {
      pred(bs.perc.percent > 0.05, bs.clone())
   }
   books.into_iter()
        .filter_map(|b: &Book| dust(to_book_share(total)(b)))
        .collect()
}

impl CsvWriter for BookShare {
   fn as_csv(&self) -> String {
      let b = &self.book;
      format!("{},{},{}", ticker(b), vol_24h(b), &self.perc)
   }
   fn ncols(&self) -> usize { 3 }
}
