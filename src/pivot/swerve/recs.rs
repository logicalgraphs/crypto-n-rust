use chrono::NaiveDate;

use book::{
   err_utils::ErrStr,
   types::Stamped
};

use crate::{
   snarf::snarf_emas,
   types::{PivotTable,Token,mk_deltas,mk_rec,confidence,Rec}
};

pub fn rec(table: &PivotTable, date: &NaiveDate, for_rows: u64,
           t1: &Token, t2: &Token) -> ErrStr<(Stamped<Rec>, Option<f32>)> {
   let emas = snarf_emas(table, date, for_rows, t1, t2)?;
   let deltas = mk_deltas(&emas);
   Ok((mk_rec(&emas),confidence(&deltas)))
}
