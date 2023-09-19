// Creates a (hierarchical) voronoi-chart from TSV-data.

// "What TSV-data?" you ask.
// currently, I'm charting protocols and ROI.
// That TVS-data.

use book::{
   file_utils::extract_date_and_body,
   list_utils::tail,
   utils::get_args
};

use crypto::types::usd::USD;

fn usage() {
   println!("./voronoi <tsv-file>");
   println!("\n\trenders a voronoi-chart of protocol and ROI-data");
}

fn main() {
   let files = get_args();
   if !files.is_empty() {
      print_prelude();
      for file in files {
         let (_date, body) = extract_date_and_body(&file);
         let protocols = tail(&body);
         buidl_arr(&protocols);
         output_js();
      }
   } else {
      usage();
   }
}

fn print_prelude() {
   let url = "https://observablehq.com/@will-r-chase/voronoi-treemap";
   println!("goto:\n{url}\n");
}

fn buidl_arr(protocols: &Vec<String>) {
   println!("protocols = [");
   protocols.iter().for_each(buidl_obj);
   println!("]");
}

fn quot(s: &str) -> String {
   format!("\"{s}\"")
}

fn buidl_obj(protocol: &String) {
   let things: Vec<&str> = protocol.split('\t').collect();
   if let [prot, blok, _, val, _gan] = things.as_slice() {
      let valu: USD =
         val.parse().expect(&format!("Not an amount: {val}"));
      let v = valu.amount;
      let (protocol, block) = (quot(prot), quot(blok));
      let f2 = format!("protocol: {protocol}, blockchain: {block}");
      println!("\t{{ {f2}, value: {v} }},");
   } else {
      println!("Could not parse line: {protocol}");
   }
}

fn output_js() {
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
    let colors = vec![
       ("FANTOM", "Fuchsia"),
       ("Avalanche", "Tomato"),
       ("Arbitrum", "Pink"),
       ("COSMOS", "SteelBlue"),
       ("HARMONY", "PaleGreen"),
       ("Optimism", "Gold")];
   for (b,c) in colors {
      println!("   {}: {},", quot(b), quot(c));
   }
  println!("  }};
  return colors[region];
}}");

}
