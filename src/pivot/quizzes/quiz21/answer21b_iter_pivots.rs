use std::collections::HashMap;

use book::{
   csv_utils::parse_csv,
   err_utils::ErrStr,
   file_utils::extract_date_and_body,
   list_utils::{ht,tail},
   num_utils::parse_num,
   utils::get_args
};

use swerve::types::{Token,mk_token};

fn usage() -> ErrStr<()> {
   println!("./answer21 <portfolio>
	Parses <portfolio>, extracting tokens and amounts
");
   Err("Must include <portfolio> file!".to_string())
}

type Blockchain = String;
type Amount = f32;
type Tokens = HashMap<Token, Amount>;

fn parse_tokens(row: &Vec<String>) -> ErrStr<Tokens> {
   let mut ans = HashMap::new();
   for window in row.chunks(2) {
      if let Some(tok) = window.get(0) {
         if tok == "" { continue; }
         let token = mk_token(&tok);
         let amt = window.get(1).ok_or(format!("No amount listed for {tok}"))?;
         let amount = parse_num(&amt)?;
         ans.insert(token, amount);
      }
   }
   Ok(ans)
}

struct Assets {
   blockchain: Blockchain,
   tokens: Tokens
}

fn asset_parser(v: Vec<String>) -> ErrStr<Assets> {
   let (h, t) = ht(&v);
   let blockchain = h.ok_or(format!("No blockchain in {v:?}"))?;
   let tokens = parse_tokens(&t)?;
   Ok(Assets { blockchain, tokens })
}

type Pools = HashMap<Blockchain, Vec<Tokens>>;

fn file_to_pools(lines: &Vec<String>) -> ErrStr<Pools> {
   let blocks = parse_csv(0, asset_parser, lines)?;
   let mut ans = HashMap::new();
   for block in blocks {
      let toks = block.tokens.clone();
      ans.entry(block.blockchain)
         .and_modify(|assets: &mut Vec<_>| assets.push(toks.clone()))
         .or_insert(vec!(toks));
   }
   Ok(ans)
}

fn main() -> ErrStr<()> {
   let args = get_args();
   if let Some(file) = args.first() {
      let (date, lines) = extract_date_and_body(&file)?;
      println!("File {file}, dated {date}, has {} items.", &lines.len());
      let pools = file_to_pools(&tail(&lines))?;
      for (blockchain, tokens) in pools {
         println!("\nFor {blockchain}, I have the following pivot-pools:\n");
         for assets in tokens {
            println!("\t{assets:?}");
         }
      }
      Ok(())
   } else {
      usage()
   }
}
