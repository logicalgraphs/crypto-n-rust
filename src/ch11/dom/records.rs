use std::{
   cmp::Ordering,
   collections::{HashMap, HashSet},
   fs::File
};

extern crate serde;

use serde_json::from_reader; // {Value, from_str}; // , from_value};
use serde::Deserialize; // ,Deserializer};

use book::{
   string_utils::bracket
};

/*
A record is:

{
    "protocol": "Indigo",
    "domains": [
      "Staking"
      "HODL",
    ],
    "tvl": 5473.7375999999995
}
*/

#[derive(Deserialize, Debug, Clone)]
struct RawRecord {
   tvl: f32,
   domains: Vec<String>
}

pub fn records(filename: &str) -> Vec<Record> {
   let file = File::open(filename)
       .expect("file should open read only");
   let raw_recs: Vec<RawRecord> = from_reader(file).expect("venn'd!");
   raw_recs.into_iter().map(raw_rec_2_rec).collect()
}

#[derive(Debug, Clone)]
pub struct Record {
   tvl: f32,
   domains: Vec<String>
}

fn raw_rec_2_rec(rr: RawRecord) -> Record {
   let set_domains: HashSet<String> = rr.domains.into_iter().collect();
   let tvl = rr.tvl;
   let mut domains: Vec<String> = set_domains.into_iter().collect();
   domains.sort();
   Record { tvl, domains }
}

pub fn records_as_string(recs: &Vec<Record>) -> String {
   let preamble = "var sets = [";
   let postlude = "];";

   format!("{preamble}\n{}{postlude}\n", as_sets(recs))
}

fn as_sets(recs: &Vec<Record>) -> String {
   let mut sets = records_2_sets(recs);

   fn compare_domains(a: &Set, b: &Set) -> Ordering {
      let da = &a.sets;
      let db = &b.sets;
      db.len().cmp(&da.len()).then(da.cmp(&db))
   }

   sets.sort_by(compare_domains);
   let sets1: Vec<String> = sets.into_iter().map(set_to_str).collect();
   sets1.join(",\n")
}

struct Set {
   sets: Vec<String>,
   tvl: f32
}

fn set_to_str(s: Set) -> String {
   format!("   {{ sets: {}, size: {} }}", as_arr(&s.sets), s.tvl)
}

fn as_arr(s: &Vec<String>) -> String {
   fn quot(s: &String) -> String { bracket("''", s) }
   let domains: Vec<String> = s.into_iter().map(quot).collect();
   bracket("[]", &domains.join(", "))
}

fn records_2_sets(recs: &Vec<Record>) -> Vec<Set> {
   let mut sets = HashMap::new();
   for rec in recs {
      add_sets(rec, &mut sets);
   }

   fn entry_as_set(entry: (Vec<String>, f32)) -> Set {
      Set { sets: entry.0, tvl: entry.1 }
   }
   sets.into_iter().map(entry_as_set).collect()
}

type SetMap = HashMap<Vec<String>, f32>;

fn add_sets(rec: &Record, sets: &mut SetMap) {
   let key = &rec.domains;
   let tvl = rec.tvl;
   let set = sets.entry(key.clone()).or_insert(0.0);
   *set += tvl;
   if key.len() > 1 { add_each_domain(&key, tvl, sets); }
}

fn add_each_domain(keys: &Vec<String>, tvl: f32, sets: &mut SetMap) {
   let len = keys.len() as f32;
   let portion = tvl / len;
   for key in keys {
      let vec_o_key = Vec::from([key.clone()]);
      let set = sets.entry(vec_o_key).or_insert(0.0);
      *set += portion;
   }
}
