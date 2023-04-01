
use std::collections::HashMap;

use book::{
   file_utils::extract_date_and_body,
   num_utils::parse_commaless,
   utils::get_args
};

use crypto::{
   types::{
      marketplace::prices,
      usd::{USD,mk_usd},
   },
   algos::orders::read_marketplace
};

#[derive(Debug, Clone)]
pub struct Token {
   token: String,
   amount: f32
}

fn mk_token(tok: &str, whole: f32, fract: f32) -> Token {
   let num = format!("{whole}.{fract}");
   let amt: Result<f32, _> = num.parse();
   if let Ok(amount) = amt {
      let token = tok.to_string();
      Token { token, amount }
   } else {
      panic!("Cannot convert {num} for {tok}")
   }
}

fn value(m: &HashMap<String, USD>) -> impl Fn(&Token)
    -> Option<(String, USD)> + '_ {
   |t| {
      let namei = t.token.clone();
      if let Some(price) = m.get(&namei) {
         Some((namei, mk_usd(price.amount * t.amount)))
      } else {
         None
      }
   }
}

fn usage() {
   println!("./wallet <market JSON> <wallet LSV>");
   println!("\nPrints your tokens and their USD-values.");
}

fn find_token(lines: &Vec<String>) -> Option<(usize, Token)> {
   for (idx, window) in lines.windows(3).enumerate() {
      if let Ok(whole) = parse_commaless(&window[1]) {
         if let Ok(fract) = parse_commaless(&window[2]) {
            return Some((idx, mk_token(&window[0], whole, fract)))
         }
      }
   }
   None
}

fn load_tokens(lines: &Vec<String>, toks: &mut Vec<Token>) {
   if let Some((idx, tok)) = find_token(lines) {
      toks.push(tok);
      let (_, new_lines) = lines.split_at(idx + 3);
      load_tokens(&new_lines.to_vec(), toks);
   }
}

fn pair(t: &Token) -> (String, f32) { (t.token.clone(), t.amount) }

fn main() {
   if let [market, wallet] = get_args().as_slice() {
      let markets = read_marketplace(market);
      let prices = prices(&markets);
      let (date, body) = extract_date_and_body(wallet);
      let mut tokens: Vec<Token> = Vec::new();
      load_tokens(&body, &mut tokens);
      let mut alphs: Vec<(String, f32)> = tokens.iter().map(pair).collect();
      alphs.sort_by(|x,y| x.0.cmp(&y.0));
      let mut chonks: Vec<(String, USD)> = 
         tokens.iter().filter_map(value(&prices)).collect();
      chonks.sort_by(|x,y| y.1.partial_cmp(&x.1).unwrap());
      let zs = alphs.iter().zip(chonks.iter());
      println!("Wallet balances on\t\t\t\t{date}\n");
      println!("asset\tbalance\t\tasset\tvalue (USD)");
      zs.for_each(|((a,b),(c,d))| println!("{a}\t{b}\t\t{c}\t{d}"));
   } else {
      usage();
   }
}
