use book::{
   csv_utils::{CsvWriter,list_csv},
   html_utils::Mode,
   list_utils::{head,ht},
   report_utils::{print_footer, print_top_of},
   utils::get_args
};

use crypto::{
   rest_utils::read_markets,
   types::books::{Book,parse_books,count}
};

use tob::{
   analyzed_books::analyze,
   linked_books::{LinkedBook,mk_linked}
};

// The skeleton upon which this get-fetch example is based is:
// https://stackoverflow.com/questions/43222429/how-do-you-make-a-get-request-in-rust#:~:text=Sending%20a%20GET%20request%20is,send().unwrap()%3B%20assert_eq!

/* original code to read from an endpoint then process in Rust:

reqwest = "0.9.18" in cargo file

let mut res = reqwest::get("https://api.kujira.app/api/coingecko/tickers")?;
let mut body = String::new();
res.read_to_string(&mut body)?;
reportage(&date, &body);
*/

fn usage() {
   println!("./top_order_books [--raw] <date>");
   println!("\tGives the top order books by volume");
   println!("\t--raw flag gives exact volumes and percent-analyses.");
}

fn main() -> Result<(), String> {
   let args = get_args();
   let mut success = false;
   if let (Some(frist), r1) = ht(&args) {
      let raw = frist == "--raw";
      if let Some(date) = if !raw { Some(frist) } else { head(&r1) } {
         success = true;
         let json = read_markets()?;
         reportage(&date, &json, raw);
      }
   }
   if !success {
      usage();
   }
   Ok(())
}

fn reportage(date: &str, body: &str, raw: bool) {
   let books = parse_books(&body);
   let mut alles: Vec<Book> = books.clone().into_iter().collect();
   alles.sort_by(|a, b| b.vol_24h.partial_cmp(&a.vol_24h).unwrap());
   if raw { print_alles(&alles, date); }
   let v: Vec<Book> = alles.clone().into_iter().take(5).collect();
   let x: Vec<Book> = alles.clone().into_iter().take(10).collect();
   let topus: Vec<Book> =
      alles.into_iter().take_while(|b| b.vol_24h > 100000.0).collect();
   println!("I got {} books; {} have $100,000+ 24h-volume, {date}",
            books.len(), topus.len());
   count(&books, "axlUSDC");
   count(&books, "USK");
   print_txt(&v, date);
   print_html(&x, date);
}

fn print_txt<T: CsvWriter>(tops: &Vec<T>, date: &str) {
   printer(tops, date, &Mode::TEXT);
   println!("\nfull report archived at ");
}

fn print_alles(alles: &Vec<Book>, date: &str) {
   println!("FIN order books by volume, {date}\n");
   println!("{}", list_csv(&analyze(alles)));
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
