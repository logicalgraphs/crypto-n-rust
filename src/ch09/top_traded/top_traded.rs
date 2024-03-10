use std::collections::HashSet;

use book::{
   err_utils::ErrStr,
   list_utils::first_last,
   utils::get_args
};

use crypto::{
   rest_utils::graphs_fin_res,
   types::{
      books::{Book,Volumes,vol_24h_pair,volumes_by_token,parse_books},
      usd::USD
   }
};

fn usage() -> ErrStr<()> {
   println!("\n./top_traded <date> [min volume=10000]\n");
   println!("Prints the top-traded tokens by 24h-volumes.\n");
   println!("The set of sets can be represented as a Venn diagram using, i.e.");
   println!("https://github.com/benfred/venn.js");
   Ok(())
}

fn main() -> ErrStr<()> {
   if let (Some(date), min) = first_last(&get_args()) {
      do_it(&date, min)
   } else {
      usage()
   }
}

fn do_it(date: &str, min_opt: Option<String>) -> ErrStr<()> {
   let (_, books) = parse_books(Some(graphs_fin_res("aliases.csv")));
   let tok_vols = volumes_by_token(&books);
   let default_min: f32 = 10000.0;
   let min: f32 =
      if let Some(mini) = min_opt {
         mini.parse().ok().or(Some(default_min)).unwrap()
      } else { default_min };

   println!("var sets = [");

   let mut toks = HashSet::new();
   books.iter().for_each(print_book(min, &mut toks));
   tok_vols.iter().for_each(print_token(min, &toks));
   println!("];");
   report(date, &toks, &tok_vols)
}

fn report(date: &str, toks: &HashSet<String>, tok_vols: &Volumes) -> ErrStr<()> {
   let topos = format!("Top 10 Tokens traded on @TeamKujira FIN, {date}");
   println!("{topos}\n");
   let mut vols: Vec<(String, USD)> = tok_vols.clone().into_iter().collect();
   vols.sort_by(|a,b| b.1.cmp(&a.1));
   let mut i: i32 = 0;
   for (tok, vol) in &vols {
      if toks.contains(tok) {
         i += 1;
         if i > 10 { break; }
         println!("{i}. {tok}: {vol}");
      }
   }

   i = 0;
   println!("\n<h3>{topos}</h3>\n<p>&nbsp;</p>\n<ol>");
   for (tok, vol) in vols {
      if toks.contains(&tok) {
         i += 1;
         if i > 10 { break; }
         println!("<li>{tok}: {vol}</li>");
      }
   }
   println!("</ol>");
   Ok(())
}

fn print_token(min: f32, toks: &HashSet<String>)
      -> impl Fn((&String, &USD)) -> () + '_ {
   move |(tok, val): (&String, &USD)| {
      if val.amount > min && toks.contains(tok) {
         println!("   {{sets: ['{tok}'], size: {}}},", val.amount);
      }
   }
}

fn print_book(min: f32, toks: &mut HashSet<String>)
      -> impl FnMut(&Book) -> () + '_ {
   move | b: &Book | {
      let ((bk, tg), vol) = vol_24h_pair(b);
      if vol.amount > min {
         println!("   {{sets: ['{bk}', '{tg}'], size: {}}},", vol.amount);
         toks.insert(bk);
         toks.insert(tg);
      }
   }
}
