use book::{
   csv_utils::{mk_csvs,columns,print_line,list_csv},
   html_utils::{h,mk_li,Mode,HTML,proff,HTML::OL,LI,AsText,a, nbsp},
   string_utils::plural
};

use crate::types::{
   LiquidationsByDate,Market,Top5s,top5s,big_generator,
   liquidations,xform,id_market,erase_asset,erase_bid
};

// ----- Printers --------------------------------------------------

pub fn report(date: &str, jours: &LiquidationsByDate) {
   report_by(date, "Market", id_market, jours, true);
   report_by(date, "Bid", erase_asset, jours, false);
   report_by(date, "Asset", erase_bid, jours, false);
}

fn report_by(date: &str, title: &str, f: impl Fn(&Market) -> Market,
             jours: &LiquidationsByDate, print_footer: bool) {
   println!("\nORCA liquidations by {title}\n");
   let basis = xform(&f, jours);
   let days = big_generator(&basis);
   let markets = liquidations(&basis);
   let top5 = top5s(&markets);
   
   println!("{}, , , , , , , ,{}", header("date"), header("n"));

   let mut cols = Vec::new();
   cols.push(mk_csvs(&days));
   cols.push(mk_csvs(&markets));

   columns(&cols, 7).iter().for_each(print_line);

   print_top5s_csv(&top5, title, date, print_footer);
   println!("");
   print_top5s_html(&top5, title, date, print_footer);
}

fn top5_header(topel: &Top5s, title: &str, date: &str) -> String {
   let n = topel.len();
   let msg = "@TeamKujira ORCA";
   let liqs = plural(n as u32, "liquidation");
   let msg1 = "by ($) for 7-days trailing";

   format!("Top {n} {msg} {liqs} {title} {msg1} {date}")
}

fn print_top5s_csv(topel: &Top5s, title: &str, date: &str, footer: bool) {
   println!("\n{}\n\n{}", top5_header(topel, title, date), list_csv(&topel));
   if footer { footer_csv(); }
}

fn print_top5s_html(topl: &Top5s, title: &str, date: &str, footer: bool) {
   fn print_html(content: HTML) {
      proff(&content, &Mode::HTML);
   }
   let lis: Vec<LI> =
      topl.into_iter().map(|t| mk_li(&t.as_text())).collect();
   print_html(h(2, &format!("By {title}")));
   print_html(h(3, &top5_header(topl, title, date)));
   print_html(nbsp());
   print_html(OL(lis));
   if footer { footer_html(); }
}

fn header(prefix: &str) -> String {
   format!("{prefix},bid,asset,n,amount ($),market")
}

fn report_url() -> String {
   let cr = "https://raw.githubusercontent.com/logicalgraphs/crypto-n-rust";
   let url = "main/data-files/ORCA/report.csv";
   format!("{cr}/{url}")
}

fn footer_csv() {
   println!("\nRaw CSV of report archived at {}", report_url());
}

fn footer_html() {
   proff(&a(&report_url(), "Raw CSV of report"), &Mode::HTML);
}
