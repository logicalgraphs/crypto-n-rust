// ./vern'

// Given current market conditions and sets of isomorphic paths,
// ./vern' finds the efficacious paths A -> B where B is the isomorphism
// and A is a node (or vertex) in paths B.

// History:

// We read in a path then compute the efficacy of trading on that path

// improvement: use only the paths and derive the ratios from marketplace
// sordid: we sort by trade results
// exposé: we exposé the interim trades

// ... also, I moved a lot of this stuff to the algos-library

use std::collections::HashSet;

use book::{
   string_utils::str_string,
   utils::get_args
};

use crypto::{
   types::{
     books::load_books,
     marketplace::{prices,merge_synthetics},
     usd::mk_usd
   },
   algos::{
      orders::{active_order_books,books_orderbooks},
      paths::{paths_processor,process_with_path,print_path}
   }
};

fn usage() {
   let m = "<marketplace JSON file>";
   let s = "<synthetics TSV file>";
   let g = "<graph paths CSV file>";
   println!("./vern ntokens start-token end-token {m} {s} {g}");
   println!("\n\tcomputes the number of tokens after trading a path.\n");
}

fn main() {
   let args = get_args();
   if args.len() < 5 { usage(); } else { go(&args); }
}

fn go(args: &Vec<String>) {
   let mut cont = false;
   let (args1, files) = args.split_at(5);
   if let [toks, stok, etok, marketplace, synt] = args1 {
      cont = !files.is_empty();
      if cont {
         println!("./vern, my main man, ./vern!\n");
         match toks.parse() {
            Ok(ntoks) => {
               paths(ntoks, marketplace, synt, etok, stok, &files);
            },
            Err(_) => { cont = false; }
         }
      }
   }

   if !cont {
      usage();
   }
}

fn str_str_str(s: &&str) -> String {
   str_string(*s)
}

fn paths(ntoks: f32, marketpl: &str, synth: &str,
         etok: &str, stok: &str, files: &[String]) {
   let books = load_books(marketpl);
   let market = books_orderbooks(&books);
   let quotes = prices(&market);
   let (vol, vol24) = if let Some(price) = quotes.get(stok) {
         let base = price.amount * ntoks;
         (base, base * 24.0)
   } else { (1000.0, 1000.0) };
   let mut lively = market.clone();
   active_order_books(&mut lively, &books, vol24);
   merge_synthetics(&mut lively, &quotes, synth);
   let pathf = |line: &String, processed: &mut HashSet<Vec<String>>| {
      let raw_path: Vec<&str> = line.split(',').collect();
      let lst: &str = etok;
      if raw_path.last() == Some(&lst) {
         let path: Vec<String> =
            raw_path.iter()
               .skip_while(|n| n != &&stok)
               .map(str_str_str)
               .collect();
         if !processed.contains(&path) {
            processed.insert(path.clone());
            process_with_path(ntoks, &lively, &path)
         } else {
            None
         }
      } else {
         None
      }
   };
   let mut processed_paths: HashSet<Vec<String>> = HashSet::new();
   for file in files {
      println!("For file {}:", &file);
      let paths = paths_processor(&pathf, file, &mut processed_paths);
      paths.iter().for_each(print_path(ntoks));
   }
   let pre = "\nHey, Ray! 😊 Given the trade was ";
   println!("{pre}{}, the top {} order books were used",
            mk_usd(vol), lively.len());
}
