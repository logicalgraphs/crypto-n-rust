use std::collections::HashSet;

use book::{
   err_utils::ErrStr,
   html_utils::{Mode,HTML,HTML::OL,LI,mk_li,proff,h},
   list_utils::first_last,
   string_utils::plural,
   utils::{get_args,pred}
};

use crypto::{
   rest_utils::graphs_fin_res,
   types::{
      books::{Book,Volumes,vol_24h_pair,volumes_by_token,parse_books},
      usd::USD
   }
};

fn usage() -> ErrStr<()> {
   println!("\n./top_traded <date> [min volume=30000]\n");
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
   let default_min: f32 = 30000.0;
   let min: f32 =
      if let Some(mini) = min_opt {
         mini.parse().ok().or(Some(default_min)).unwrap()
      } else { default_min };

   println!("var sets = [");

   let mut toks = HashSet::new();
   books.iter().for_each(print_book(min, &mut toks));
   tok_vols.iter().for_each(print_token(min, &toks));
   println!("];");
   report(date, &toks, tok_vols);
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

fn report(date: &str, toks: &HashSet<String>, tok_vols: Volumes) {
   let mut vols: Vec<(String, USD)> = tok_vols.into_iter().collect();
   vols.sort_by(|a,b| b.1.cmp(&a.1));

   fn contfor(toks: &HashSet<String>)
         -> impl Fn((String, USD)) -> Option<LI> + '_ {
      | (tok, vol): (String, USD) |
         pred(toks.contains(&tok), mk_li(&format!("{tok}: {vol}")))
   }
   let toppers0: Vec<LI> = vols.into_iter().filter_map(contfor(toks)).collect();
   let toppers: Vec<LI> = toppers0.into_iter().take(10).collect();
   let sz = &toppers.len();
   let tops = OL(toppers);

   print_report(date, &tops, &Mode::TEXT, *sz);
   print_report(date, &tops, &Mode::HTML, *sz);
}

fn header(date: &str, n: usize) -> HTML {
   let t = plural(n, "Token");
   h(3, &format!("Top {} traded on @TeamKujira FIN, {date}", t))
}

fn print_report(date: &str, tops: &HTML, mode: &Mode, sz: usize) {
   proff(&header(date, sz), &mode);
   proff(&tops, &mode);
   println!("");
}
