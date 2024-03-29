use std::collections::HashMap;

use book::tuple_utils::{fst,snd};

use crate::types::{
   interfaces::{Book,Books,VPair,vols},
   pairs::untag,
   usd::{USD,no_monay}
};

// ----- Volumes -------------------------------------------------------

pub type Volumes = HashMap<String, USD>;

pub fn volumes_by_token(bs: &Books) -> Volumes {
   let mut ans: Volumes = HashMap::new();
   fn put<'a>(v: &'a mut Volumes, mut f: impl FnMut((VPair, VPair)) -> VPair + 'a)
         -> impl FnMut(&Book) -> () + 'a {
      move |b: &Book| {
         let (bk, bv) = untag(&f(vols(b)));
         let bas = v.entry(bk).or_insert(no_monay());
         *bas += bv;
      }
   }
   bs.into_iter().for_each(put(&mut ans, fst));
   bs.into_iter().for_each(put(&mut ans, snd));
   ans
}
