// extracts current market data from 
// https://api.kujira.app/api/coingecko/tickers

use crate::types::{
   aliases::{Aliases,alias},
   interfaces::{Books,Book,mk_book,Prices,Price,get_price},
   internal::types::{Books1,Book1},
   usd::{USD,mk_usd}
};

// ----- Parsing -------------------------------------------------------

pub fn books2books(p: &Prices, bs: &Books1, aliases: &Aliases) -> Books {
   fn price_tokens<'a>(p: &'a Prices, a: &'a Aliases)
         -> impl Fn(&Book1) -> Option<Book> + 'a {
      | b0 | {
         let bas = alias(a, &b0.base);
         p.get(&bas).and_then(|b_price| {
            let tar = alias(a, &b0.target);
            p.get(&tar).and_then(|t_price| {
               fn vol(p: &Price, t: f32) -> USD {
                  mk_usd(get_price(&p).amount * t)
               }
               Some(mk_book(bas,tar, b0.pool_id.clone(),
                            vol(b_price, b0.base_vol),
                            vol(t_price, b0.target_vol), b0.last))
            })
         })
      }
   }
   bs.into_iter().filter_map(price_tokens(p, aliases)).collect()
}
