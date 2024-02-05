use book::{
   csv_utils::{Indexed,mk_csvs,columns,print_line,print_csv},
   string_utils::plural
};

use crate::types::{
   LiquidationsByDate,Market,Top5,big_generator,top5s,
   liquidations,xform,id_market,erase_asset,erase_bid
};

// ----- Printers --------------------------------------------------

pub fn report(date: &str, jours: &LiquidationsByDate) {
   report_by(date, "Market", id_market, jours);
   report_by(date, "Bid", erase_asset, jours);
   report_by(date, "Asset", erase_bid, jours);
}

fn report_by(date: &str, title: &str, f: impl Fn(&Market) -> Market,
             jours: &LiquidationsByDate) {
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

   print_top5s(&top5, title, date);

   footer();
}

fn print_top5s(topel: &Vec<Indexed<Top5>>, title: &str, date: &str) {
   let n = topel.len();
   let msg = "@TeamKujira ORCA";
   let liqs = plural(n as u32, "liquidation");
   let msg1 = "by ($) for 7-days trailing";

   println!("\nTop {n} {msg} {liqs} {title} {msg1} {date}\n");
   topel.iter().for_each(print_csv);
}

fn header(prefix: &str) -> String {
   format!("{prefix},bid,asset,n,amount ($),market")
}

fn footer() {
   let cr = "https://raw.githubusercontent.com/logicalgraphs/crypto-n-rust/";
   let url = "main/data-files/ORCA/report.csv";
   println!("\nRaw CSV of report archived at {cr}{url}");
}
