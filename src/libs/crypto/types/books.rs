// extracts current market data from 
// https://api.kujira.app/api/coingecko/tickers

use std::collections::HashSet;

use book::{
   err_utils::ErrStr,
   string_utils::parse_lines,
   stream_utils::lines_from_stream,
};

use crate::{
   rest_utils::graphs_fin_res,
   types::{
      aliases::load_aliases,
      interfaces::{Books,Book,BookBooks,mk_book,trades_token,vol_24h},
      internal::{
         books::books2books,
         prices::prices_from_books,
         types::raw_books
      },
      usd::USD
   }
};

pub fn parse_books(date: &str, opt_aliases: Option<String>) -> BookBooks {
   let b0 = raw_books();
   let aliases = load_aliases(&opt_aliases);
   let p = prices_from_books(&date, &b0, &aliases);
   let b = books2books(&p, &b0, &aliases);
   (p, b)
}  
   
pub fn parse_books_with_aliases(date: &str) -> BookBooks {
   parse_books(&date, Some(graphs_fin_res("aliases.csv")))
}  

/* 
A special case treating protocols-blockchains or tokens-blockchains
as 'order books' because they have the same structure: the end-game is
to structure these data as a Venn Diagram, or a graph, ... or both.

Or Voronoi? That will clean up some code.

Will I get burned by this semantic overloading?

Probably? Maybe?

But let's go with this for now and adapt as the end-game clarifies.

The structure of these 'books' are as follows, a TSV-file of the form:

blah-di-blah,some other stuff,date

protocol/token,blockchain,_invested,value $,other stuff,...

e.g.:

Blockaverse	portfolio	2024-03-15					
							
Token	Blockchain	invested	value	gain/loss	ROI	real token name	rôle
GMX	Arbitrum	$933.00	$682.45	-$250.55	-26.85%	GMX	
BTC	Cardano	$409.29	$867.55	$458.26	111.97%	iBTC	blue-chip
*/

pub fn load_books_from_stream() -> ErrStr<Books> {
   let lines = lines_from_stream();
   fn parser(line: String) -> ErrStr<Book> {
      let cols0: Vec<&str> = line.split("\t").collect();
      let cols: Vec<&str> = cols0.into_iter().take(4).collect();
      if let [tok, block, _, val] = cols.as_slice() {
         let u: USD =
            val.parse()
               .expect(&format!("Could not parse dollar value: '{val}'"));
         Ok(mk_book(tok.to_string(), block.to_string(), "".to_string(), 
                    u.clone(), u, 0.0))
      } else {
         Err(format!("Could not parse line: '{line}'"))
      }
   }
   let books: HashSet<Book> =
      parse_lines(parser, &lines, Some(3))?.into_iter().collect();
   Ok(books)
}

pub fn fetch_books(fin: &Books, token: &str) -> Books {
   book_fetcher(trades_token(&token), fin)
}

pub fn fetch_books_by_vol(fin: &Books, vol: USD) -> Books {
   book_fetcher(move |b: &Book| vol_24h(b) > vol, fin)
}

fn book_fetcher(f: impl Fn(&Book) -> bool, fin: &Books) -> Books {
   let mut q = HashSet::new();
   for b in fin {
      if f(b) { q.insert(b.clone()); }
   }
   q
}
