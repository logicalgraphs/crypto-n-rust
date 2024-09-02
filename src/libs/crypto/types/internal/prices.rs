// extracts current market data from 
// https://api.kujira.app/api/coingecko/tickers

use std::collections::{HashMap,HashSet};

use book::{
   types::untag,
   utils::pred
};

use crate::types::{
   internal::types::{Book1,Books1},
   aliases::{Aliases,alias},
   interfaces::{Prices,Price,mk_price},
   usd::{USD,mk_usd}
};

// a new take on prices

// FRIST! I load all axlUSDC prices (where prices are over $0.00)
// then I overlay with USDC-prices and USK-prices (after converting USK
// to axlUSDC-equivalent).

// THEN I take the remaining order books and ratio their prices from base
// price. Maybe I could just oracle everything, instead?

pub fn prices_from_books(date: &str, books: &Books1, aliases: &Aliases)
      -> Prices {
   let (stables, unstables) = stable_books(&date, books, aliases);
   let (axls, others) =
       books_for(&date, "axlUSDC", (&stables, &unstables), aliases);
   let (usdcs, tail) = books_for(&date, "USDC", (&stables, &others), aliases);
   let (usks, rest) = books_for(&date, "USK", (&stables, &tail), aliases);
   let prices = usdcs.into_iter()
                     .chain(usks)
                     .chain(axls)
                     .chain(stables)
                     .collect();  // please note:
                                  // for HashMap, chain() is not associative.
                                  // This means the LAST map I chain is the
                                  // MOST IMPORTANT for prices.

   // now the rest are fun!!! These are the order books that don't have a
   // stable target, SO! we need to use the prices-HashMap to find the price
   // of the target to compute the price of the base.

   let baros: Prices = rest.iter()
       .filter_map(barometric_board(&date, &prices, aliases))
       .collect();
   baros.into_iter().chain(prices).collect()
}

type VHashes<T> = (HashSet<T>, HashSet<T>);
type Book1Books = (Prices, HashSet<Book1>);
type Book1BooksRef<'a> = (&'a Prices, &'a Books1);

fn part(f: impl Fn(&Book1) -> String, v: &Books1, p: &str) -> VHashes<Book1> {

   // why am I writing: v.into_iter().partition(|b| b.target == p)
   // in long-form? f'n copy-semantics and Rust, stg.

   let mut left = HashSet::new();
   let mut right = HashSet::new();
   for b in v {
      (if &f(b) == p { &mut left } else { &mut right }).insert(b.clone());
   }
   (left, right)
}

// only consider prices from books that have had trades today ... functionally!

fn d(d1: &str, u: USD) -> Price { mk_price(d1, u) }

fn mb_book<'a>(date: &'a str, factor: &'a USD, a: &'a Aliases)
      -> impl Fn(&Book1) -> Option<(String, Price)> + 'a {
   move | b: &Book1 | {
      pred(b.last > 0.0 && b.target_vol + b.base_vol > 0.0,
           (alias(a, &b.base), d(&date, mk_usd(b.last * factor.amount))))
   }
}

fn books_for(date: &str, stable: &str, (stables, books): Book1BooksRef,
             aliases: &Aliases) -> Book1Books {
   let (mines, yourses) =
      part(move |b: &Book1| alias(aliases, &b.target), books, stable);

   fn mk_books(d: &str, dollah: &USD, src: &Books1, a: &Aliases) -> Prices {
      src.into_iter().filter_map(mb_book(&d, dollah, a)).collect()
   }
   let quote = untag(stables.get(stable).unwrap()).1;
   (mk_books(date, &quote, &mines, aliases), yourses)
}

fn stable_books(date: &str, books: &Books1, a: &Aliases) -> Book1Books {
   let (stables, unstables) =
      part(|b: &Book1| alias(a, &b.base), books, "axlUSDC");
   let mut books = HashMap::new();
   for s in stables {
      books.insert(alias(a, &s.target), d(&date, compute_stable_price(&s)));
   }
   books.insert("axlUSDC".to_string(), d(&date, mk_usd(1.0)));
   // just how I rollz, yo!
   (books, unstables)
}

// Here, we take the books that don't have a stable target, or so I think, then
// compute the prices for the bases to round out the token-prices-list.

fn barometric_board<'a>(date: &'a str, prices: &'a Prices, a: &'a Aliases)
         -> impl Fn(&Book1) -> Option<(String, Price)> + 'a {
   fn mb_price<'b>(d: &'b str, b: &'b Book1, a: &'b Aliases)
          -> impl Fn(&Price) -> Option<(String, Price)> + 'b {
      move |price| { 
         let p = untag(price).1;
         let x = mb_book(d, &p, a)(b);
         x   // <-- so to live on through the borrow
      }
   }
   |book: &Book1|
      prices.get(&alias(a, &book.target)).and_then(mb_price(date, book, a))
}

fn compute_stable_price(b: &Book1) -> USD { mk_usd(1.0 / b.last) }

// Now that we have order-book-volumes-by-token and token-prices, we can
// compute order-book-volumes(-by-price) on the active order books, returning
// the active order books, only.
