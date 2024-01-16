use book::csv_utils::{mk_csvs,columns};

use crate::types::{
   LiquidationsByDate,Market,big_generator,top5s,
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
   
   let msg = "Top 5 @TeamKujira ORCA liquidations by";
   let msg1 = "($) for 7-days trailing";

   println!("{}, ,{}, ,{msg} {title} {msg1} {date}",
            header("date,"), header(""));

   let mut cols = Vec::new();
   cols.push(mk_csvs(&days));
   cols.push(mk_csvs(&markets));
   cols.push(mk_csvs(&top5));

   columns(&cols).iter().for_each(|line| println!("{}", line));

   footer();
   separate();
}

fn header(prefix: &str) -> String {
   format!("{prefix}bid,asset,n,amount ($),market")
}

fn footer() {
   let cr = "https://raw.githubusercontent.com/logicalgraphs/crypto-n-rust/";
   let url = "main/data-files/ORCA/report.csv";
   println!("\nRaw CSV of report archived at {cr}{url}");
}

fn separate() {
   for _i in 1..21 {
      println!("");
   }
}
