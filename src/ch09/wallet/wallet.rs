use std::{
   collections::HashMap,
   fmt,
   slice::Iter
};

use strum::IntoEnumIterator;

use book::{
   file_utils::extract_date_and_body,
   html_utils::{p,a,nbsp,h,body,Mode,proff,roff},
   num_utils::parse_commaless,
   utils::get_args
};

use crypto::{
   types::{
      marketplace::prices,
      usd::{USD,mk_usd,no_monay},
   },
   algos::orders::read_marketplace
};

use wallet::tsv::TsvWriter;

#[derive(Debug, Clone)]
struct Pair<T> {
   k: String,
   v: T
}

fn mk_pair<T: Clone>(key: &str, val: T) -> Pair<T> {
   Pair { k: key.to_string(), v: val.clone() }
}

impl Default for Pair<USD> {
   fn default() -> Self {
      mk_pair("_", no_monay())
   }
}

impl<T: fmt::Display> TsvWriter for Pair<T> {
   fn as_tsv(&self) -> String { format!("{}\t{}", self.k, self.v) }
}

#[derive(Debug, Clone)]
pub struct Token {
   token: String,
   amount: f32
}

impl TsvWriter for Token {
   fn as_tsv(&self) -> String { format!("{}\t{}", self.token, self.amount) }
}

fn scan_token(tok: &str, whole: f32, fract: f32) -> Token {
   let num = format!("{whole}.{fract}");
   let amt: Result<f32, _> = num.parse();
   if let Ok(amount) = amt {
      let token = tok.to_string();
      Token { token, amount }
   } else {
      panic!("Cannot convert {num} for {tok}")
   }
}

impl Default for Token {
   fn default() -> Self {
      scan_token("", 0.0, 0.0)
   }
}

fn value(m: &HashMap<String, USD>)
    -> impl Fn(&Token) -> Option<Pair<USD>> + '_ {
   |t| {
      let namei = &t.token;
      if let Some(price) = m.get(namei) {
         Some(mk_pair(namei, mk_usd(price.amount * t.amount)))
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
            return Some((idx, scan_token(&window[0], whole, fract)))
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

fn pair(t: &Token) -> Pair<f32> { mk_pair(&t.token, t.amount) }

struct InfArr<T> {
   basis: Vec<T>
}

fn mk_inf<T: Clone>(v: &Vec<T>) -> InfArr<T> {
   InfArr { basis: v.clone() }
}

struct InfArrIter<'a, T> {
   itr: Iter<'a, T>
}

impl<T> InfArr<T> {
   fn iter(&self) -> InfArrIter<'_, T> {
      InfArrIter { itr: self.basis.iter() }
   }
}

impl<'a> Iterator for InfArrIter<'a, Pair<USD>> {
   type Item = Pair<USD>;
   fn next(&mut self) -> Option<Self::Item> {
      let mut ans = Pair::default();
      if let Some(a) = self.itr.next() {
         ans = a.clone()
      }
      Some(ans)
   }
}

fn main() {
   if let [market, wallet] = get_args().as_slice() {
      let markets = read_marketplace(market);
      let prices = prices(&markets);
      let (date, body) = extract_date_and_body(wallet);
      let mut tokens: Vec<Token> = Vec::new();
      load_tokens(&body, &mut tokens);
      let mut alphs: Vec<Pair<f32>> = tokens.iter().map(pair).collect();
      alphs.sort_by(|x,y| x.k.cmp(&y.k));
      let mut chonks: Vec<Pair<USD>> = 
         tokens.iter().filter_map(value(&prices)).collect();
      chonks.sort_by(|x,y| y.v.partial_cmp(&x.v).unwrap());
      let plonks = mk_inf(&chonks);
      let zs = alphs.iter().zip(plonks.iter());
      println!("Wallet balances on\t\t\t\t{date}\n");
      println!("asset\tbalance\t\tasset\tvalue (USD)");
      zs.for_each(|(a,b)| println!("{}\t\t{}", a.as_tsv(), b.as_tsv()));
      infos(&date);
   } else {
      usage();
   }
}

fn infos(date: &str) {
   let lg = "https://github.com/logicalgraphs/crypto-n-rust/blob";
   let src = "main/src/ch09/wallet/wallet.rs";
   let wallet_src = a(&format!("{lg}/{src}"), "./wallet");
   let kujira_wallet_url = a("https://blue.kujira.app/wallet",
                           "Kujira BLUE wallet");
   let msg = "computes and sorts balances from a scrap of";
   let title = format!("Wallet balances on {date}");
   for mode in Mode::iter() {
      let w1 = roff(&wallet_src, &mode);
      let w2 = roff(&kujira_wallet_url, &mode);
      let webby = body(&vec![h(2, &title), nbsp(),
                            p(&format!("{w1} {msg} {w2}"))]);
      proff(&webby, &mode);
      println!("");
   }
}
