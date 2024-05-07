use book::{
   html_utils::{Mode,HTML,HTML::OL,HTML::A,HTML::P,LI,mk_li,proff,roff,h},
   list_utils::first_last,
   num_utils::{mk_estimate,parse_or},
   string_utils::plural,
   utils::get_args
};

use crypto::{
   algos::orders::working_set,
   charts::venn::venn_diagram,
   types::{
      books::parse_books_with_aliases,
      usd::USD,
      volumes::Volumes
   }
};

fn usage() {
   println!("\n./top_traded <date> [min volume=50000]\n");
   println!("Prints the top-traded tokens by 24h-volumes.\n");
   println!("The set of sets can be represented as a Venn diagram using, i.e.");
   println!("https://github.com/benfred/venn.js");
}

fn main() {
   if let (Some(date), min) = first_last(&get_args()) {
      do_it(&date, min.as_ref());
   } else {
      usage();
   }
}

fn do_it(date: &str, min_opt: Option<&String>) {
   let (_, books) = parse_books_with_aliases(&date);
   let default_min: f32 = 50000.0;
   let min = parse_or(min_opt, default_min);
   let (vols, toks) = working_set(min, &books);
   println!("{}", venn_diagram((&vols, &toks)));
   report(date, vols);
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
   let pmode = |h: &HTML| proff(h, &mode);
   pmode(&header(date, sz));
   pmode(&tops);
   pmode(&footer(&mode));
   println!("");
}

fn footer(mode: &Mode) -> HTML {
   let arr = |url: &str, text: &str|
      roff(&mode, &A((url.to_string(), text.to_string())));
   let preable = "Report generated from @TeamKujira";
   let tickers = arr("https://api.kujira.app/api/coingecko/tickers",
                     "/tickers REST endpoint");
   let lg = "https://github.com/logicalgraphs/crypto-n-rust/tree/main/src";
   let url = format!("{lg}/ch09/top_traded");
   let tt = arr(&url, "my <code>./top_traded</code> system");
              
   P(format!("{preable} {tickers} with {tt}."))
}
