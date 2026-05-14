use book::{
   html_utils::{
      Mode,HTML,HTML::OL,HTML::A,HTML::P,LI,mk_li,proff,roff,h,HTML::CODE
   },
   num_utils::mk_estimate,
   string_utils::plural
};

use crypto::types::{usd::USD,volumes::Volumes};

pub fn report(date: &str, tok_vols: Volumes) {
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
   let code = roff(&mode, &CODE("./top_traded".to_string()));
   let tt = arr(&url, &format!("my {code} system"));
              
   P(format!("{preable} {tickers} with {tt}."))
}
