// Give Venn Diagram-representation as per https://github.com/benfred/venn.js

use book::{
   currency::usd::USD,
   json_utils::AsJSON,
   types::{ dyadic::{Dyad,unpair}, tagged::{Tag,mk_tag,untag} }
};

use crate::types::{ interfaces::{Books,vol_24h_pair}, volumes::Volumes };

struct DyadUSD { d: Dyad<USD> }

fn mk_d(d: Dyad<USD>) -> DyadUSD { DyadUSD { d } }

impl AsJSON for DyadUSD {
   fn as_json(&self) -> String {
      let ((bk, tg), vol) = unpair(&self.d);
      format!("   {{sets: ['{bk}', '{tg}'], size: {}}}", vol.amount())
   }
}

struct MonadUSD { m: Tag<USD> }

fn mk_m(m: Tag<USD>) -> MonadUSD { MonadUSD { m } }

impl AsJSON for MonadUSD {
   fn as_json(&self) -> String {
      let (tok, val) = untag(&self.m);
      format!("   {{sets: ['{tok}'], size: {}}}", val.amount())
   }
}

pub fn venn_diagram(date: &str, (vols, toks): (&Volumes, &Books)) -> String {
   fn jsonify<I, J>(i: I) -> Vec<String>
         where I: Iterator<Item = J>, I::Item: AsJSON {
      i.map(|m| m.as_json()).collect()
   }
   let x: Vec<String> =
      jsonify(toks.into_iter().map(|b| mk_d(vol_24h_pair(&b))));
   let y: Vec<String> =
      jsonify(vols.clone().into_iter().map(|(a,b)| mk_m(mk_tag(&a,b))));
   fn j(v: Vec<String>) -> String { v.join(",\n") }
   format!("date = '{date}';\n\nsets = [{},\n{}];", j(x), j(y))
}
