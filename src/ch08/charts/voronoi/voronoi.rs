// Creates a (hierarchical) voronoi-chart from TSV-data.

// "What TSV-data?" you ask.
// currently, I'm charting protocols and ROI.
// That TVS-data.

use std::collections::HashMap;

use book::{
   file_utils::lines_from_file,
   list_utils::{ht,tail},
   utils::get_args
};

use crypto::types::usd::USD;

use voronoi::colors::colors;

fn usage() {
   println!("./voronoi <color-palette> <tsv-file1> ...");
   println!("\n\trenders a voronoi-chart of protocol and ROI-data");
}

fn main() {
   if let (Some(colours), files) = ht(&get_args()) {
      let mut palette = colors(&colours, 10);
      print_prelude();
      for file in files {
         let lines = lines_from_file(&file);
         let (_date, body) = lines.split_at(2);
         let bod: Vec<String> = body.to_vec();
         let protocols = tail(&bod);
         let wheel = buidl_arr(&protocols, &mut palette);
         output_js(&wheel);
      }
   } else {
      usage();
   }
}

fn print_prelude() {
   let url = "https://observablehq.com/@will-r-chase/voronoi-treemap";
   println!("goto:\n{url}\n");
}

fn buidl_arr(protocols: &Vec<String>, palette: &mut Vec<String>)
   -> HashMap<String, String> {
   let mut wheel = HashMap::new();
   println!("protocols = [");
   for line in protocols {
      wheel.entry(buidl_obj(line)).or_insert_with(|| { palette.pop().unwrap() });
   }
   println!("]");
   wheel
}

fn quot(s: &str) -> String {
   format!("\"{s}\"")
}

fn buidl_obj(protocol: &String) -> String {
   let things: Vec<&str> = protocol.split('\t').collect();
   if let [prot, blok, _, val, _gan] = things.as_slice() {
      let valu: USD =
         val.parse().expect(&format!("Not an amount: {val}"));
      let v = valu.amount;
      let (protocol, block) = (quot(prot), quot(blok));
      let f2 = format!("protocol: {protocol}, blockchain: {block}");
      println!("\t{{ {f2}, value: {v} }},");
      block
   } else {
      panic!("Could not parse line: {protocol}");
   }
}

fn output_js(colors: &HashMap<String, String>) {
   let frn = quot("freedom_nest");
   println!("protocols_nested = {{
   let freedom_nest = d3.nest()
      .key(d => d.blockchain)
      .entries(protocols)
  return {{key: {frn}, values: freedom_nest}}
}}

protocol_hierarchy = d3.hierarchy(protocols_nested, d => d.values)
                       .sum(d => d.value)");

   println!("*** Remember to replace all d.population with d.value!");
   println!("*** REPLACE population_hierarchy with protocol_hierarchy!");
   println!("*** REPLACE d.data.country with d.data.protocol!");
   println!("*** REPLACE population opacity with 250!");
   println!("*** ADD '$'+ in front of bigFormat(");

   println!("REPLACE region_colors with:

regionColor = function(region) {{
  var colors = {{");
   for (b,c) in colors {
      println!("   {}: {},", b, quot(c));
   }
  println!("  }};
  return colors[region];
}}");

}
