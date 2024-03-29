// Give Venn Diagram-representation as per https://github.com/benfred/venn.js

use book::json_utils::AsJSON;

use crate::{
   types::{
      interfaces::Books,
      pairs::{Dyad,unpair,Tag,mk_tag,untag},
      usd::USD,
      volumes::{Volumes,vol_24h_pair}
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

pub fn venn_diagram((vols, toks): (&Volumes, &Books)) -> String {
   let j: Vec<String> =
      toks.into_iter().map(|b| mk_d(vol_24h_pair(&b)).as_json()).collect();
   let k: Vec<String> =
      vols.clone().into_iter().map(|p| mk_m(mk_tag(p)).as_json()).collect();
   format!("var sets = [{},\n{}];", j.join(",\n"), k.join(",\n"))
}
