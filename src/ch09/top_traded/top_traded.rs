// use std::collections::HashSet;

use book::{
   html_utils::{Mode,HTML,HTML::OL,LI,mk_li,proff,h},
   json_utils::AsJSON,
   list_utils::first_last,
   num_utils::mk_estimate,
   string_utils::plural,
   utils::get_args
};

use crypto::{
   rest_utils::graphs_fin_res,
   algos::orders::working_set,
   types::{
      books::{Volumes,vol_24h_pair,parse_books},
      pairs::{unpair,Dyad,untag,Tag,mk_tag},
      usd::USD
   }
};

fn usage() {
   println!("\n./top_traded <date> [min volume=30000]\n");
   println!("Prints the top-traded tokens by 24h-volumes.\n");
   println!("The set of sets can be represented as a Venn diagram using, i.e.");
   println!("https://github.com/benfred/venn.js");
}

fn main() {
   if let (Some(date), min) = first_last(&get_args()) {
      do_it(&date, min);
   } else {
      usage();
   }
}

fn do_it(date: &str, min_opt: Option<String>) {
   let (_, books) = parse_books(Some(graphs_fin_res("aliases.csv")));
   let default_min: f32 = 30000.0;
   let min: f32 =
      (min_opt.and_then(|mini| mini.parse().ok())).unwrap_or(default_min);

   let (vols, toks) = working_set(min, &books);
   println!("var sets = [");

   let j: Vec<String> =
      toks.into_iter().map(|b| mk_d(vol_24h_pair(&b)).as_json()).collect();
   let k: Vec<String> =
      vols.clone().into_iter().map(|p| mk_m(mk_tag(p)).as_json()).collect();
   println!("{},\n{}];", j.join(",\n"), k.join(",\n"));
   report(date, vols);
}

struct DyadUSD { d: Dyad<USD> }

fn mk_d(d: Dyad<USD>) -> DyadUSD { DyadUSD { d } }

impl AsJSON for DyadUSD {
   fn as_json(&self) -> String {
      let ((bk, tg), vol) = unpair(&self.d);
      format!("   {{sets: ['{bk}', '{tg}'], size: {}}}", vol.amount)
   }
}

struct MonadUSD { m: Tag<USD> }

fn mk_m(m: Tag<USD>) -> MonadUSD { MonadUSD { m } }

impl AsJSON for MonadUSD {
   fn as_json(&self) -> String {
      let (tok, val) = untag(&self.m);
      format!("   {{sets: ['{tok}'], size: {}}}", val.amount)
   }
}

fn report(date: &str, tok_vols: Volumes) {
   let mut vols: Vec<(String, USD)> = tok_vols.into_iter().collect();
   vols.sort_by(|a,b| b.1.cmp(&a.1));

   fn contfor((tok, vol): (String, USD)) -> LI {
      mk_li(&format!("{tok}: ${}", mk_estimate(vol.amount)))
   }
   let toppers0: Vec<LI> = vols.into_iter().map(contfor).collect();
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
