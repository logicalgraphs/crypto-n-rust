// Give Venn Diagram-representation as per https://github.com/benfred/venn.js

use book::json_utils::AsJSON;

use crate::{
   types::{
      interfaces::{Books,vol_24h_pair},
      pairs::{Dyad,unpair,Tag,mk_tag,untag},
      usd::USD,
      volumes::Volumes
   }
};

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

pub fn venn_diagram(date: &str, (vols, toks): (&Volumes, &Books)) -> String {
   let x: Vec<String> =
      toks.into_iter().map(|b| mk_d(vol_24h_pair(&b)).as_json()).collect();
   let y: Vec<String> =
      vols.clone().into_iter().map(|p| mk_m(mk_tag(p)).as_json()).collect();
   fn j(v: Vec<String>) -> String { v.join(",\n") }
   format!("date = '{date}';\n\nsets = [{},\n{}];", j(x), j(y))
}
