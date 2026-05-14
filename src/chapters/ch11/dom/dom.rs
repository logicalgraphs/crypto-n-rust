use book::utils::get_args;

use domains::records::{records, records_as_string};

fn main() {
   let args = get_args();
   if let Some(filename) = args.first() {
     let recs = records(&filename);
     println!("{}", records_as_string(&recs));
   } else {
     usage();
   }
}

fn usage() {
   println!("./dom <domains.tsv>\n");
   println!("Converts neo4j domains into sets, viewable as Venn diagrams.");
}
