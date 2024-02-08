use book::{
   csv_utils::{CsvWriter,list_csv},
   html_utils::Mode,
   report_utils::{print_footer, print_top_of}
};

use crypto::types::books::{Book,parse_books,count};

use crate::{
   analyzed_books::analyze,
   linked_books::{LinkedBook,mk_linked}
};

pub fn reportage(date: &str, body: &str, raw: Option<f32>) {
   let books = parse_books(&body);
   let nbooks = books.len();
   let mut alles: Vec<Book> = books.clone().into_iter().collect();
   alles.sort_by(|a, b| b.vol_24h.partial_cmp(&a.vol_24h).unwrap());
   let x: Vec<Book> = alles.clone().into_iter().take(10).collect();
   let v: Vec<Book> = x.clone().into_iter().take(5).collect();
   if let Some(min) = raw { print_alles(&alles, date, min); }
   alles.retain(|b| b.vol_24h > 100000.0);
   let ntop = alles.len();

   println!("I got {nbooks} books; {ntop} have $100,000+ 24h-volume, {date}");
   count(&books, "axlUSDC");
   count(&books, "USK");
   count(&books, "USDC");
   print_txt(&v, date);
   print_html(&x, date);
}

fn print_txt<T: CsvWriter>(tops: &Vec<T>, date: &str) {
   printer(tops, date, &Mode::TEXT);
   println!("\nfull report archived at \n");
}

fn print_alles(alles: &Vec<Book>, date: &str, min: f32) {
   let mut ballz = alles.clone();
   ballz.retain(|b| b.vol_24h > min);
   println!("FIN order books by volume, {date}\n");
   println!("{}\n", list_csv(&analyze(&ballz)));
}

fn foot(mode: &Mode) {
   print_footer(mode, "src/ch09/top_order_books", "top_order_books");
}

fn print_html(tops: &Vec<Book>, date: &str) {
   let linkies: Vec<LinkedBook> = tops.into_iter().map(mk_linked).collect();
   printer(&linkies, date, &Mode::HTML);
}

fn printer<T: CsvWriter>(tops: &Vec<T>, date: &str, mode: &Mode) {
   print_top_of("FIN order books by volume", date, tops, mode);
   foot(mode);
}
