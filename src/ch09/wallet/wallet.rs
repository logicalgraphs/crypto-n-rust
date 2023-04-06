use strum::IntoEnumIterator;

use book::{
   file_utils::extract_date_and_body,
   html_utils::{p,a,nbsp,h,body,Mode,proff,roff},
   utils::get_args
};

use crypto::{
   types::{
      marketplace::prices,
      usd::USD,
   },
   algos::orders::read_marketplace
};

use wallet::{
   inf_iter::mk_inf,
   pairs::Pair,
   tokens::{Token,find_token,token_pair,token_value},
   tsv::TsvWriter
};

fn usage() {
   println!("./wallet <market JSON> <wallet LSV>");
   println!("\nPrints your tokens and their USD-values.");
}

fn load_tokens(lines: &Vec<String>, toks: &mut Vec<Token>) {
   if let Some((idx, tok)) = find_token(lines) {
      toks.push(tok);
      let (_, new_lines) = lines.split_at(idx + 3);
      load_tokens(&new_lines.to_vec(), toks);
   }
}

fn main() {
   if let [market, wallet] = get_args().as_slice() {
      let markets = read_marketplace(market);
      let prices = prices(&markets);
      let (date, body) = extract_date_and_body(wallet);
      let mut tokens: Vec<Token> = Vec::new();
      load_tokens(&body, &mut tokens);
      let mut alphs: Vec<Pair<f32>> = tokens.iter().map(token_pair).collect();
      alphs.sort_by(|x,y| x.k.cmp(&y.k));
      let mut chonks: Vec<Pair<USD>> = 
         tokens.iter().filter_map(token_value(&prices)).collect();
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
