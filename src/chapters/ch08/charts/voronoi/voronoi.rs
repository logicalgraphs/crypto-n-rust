// Creates a (hierarchical) voronoi-chart from TSV-data.

// "What TSV-data?" you ask.
// currently, I'm charting protocols and ROI.
// That TVS-data.

use std::collections::HashMap;

use book::{
   err_utils::ErrStr,
   string_utils::quot,
   utils::get_args
};

use crypto::types::{
   books::{Book,Books,load_books_from_stream,vol_24h_pair},
   pairs::unpair
};

use voronoi::colors::colors;

fn usage() {
   println!("$ echo '<tsv file>' | ./voronoi <color-palette>");
   println!("\n\trenders a voronoi-chart of protocol and ROI-data");
}

fn main() -> ErrStr<()> {
   if let Some(colours) = get_args().first() {
      let mut palette = colors(&colours, 50)?;
      print_prelude();
      let protocols = load_books_from_stream()?;
      let wheel = buidl_arr(&protocols, &mut palette);
      output_js(&wheel);
   } else {
      usage();
   }
   Ok(())
}

fn print_prelude() {
   let url = "https://observablehq.com/@will-r-chase/voronoi-treemap";
   println!("goto:\n{url}\n");
}

fn buidl_arr(protocols: &Books, palette: &mut Vec<String>)
      -> HashMap<String, String> {
   let mut wheel = HashMap::new();
   println!("protocols = [");
   for prot in protocols {
      wheel.entry(buidl_obj(prot)).or_insert_with(|| {
         match palette.pop() {
            Some(c) => c,
            _ => panic!("Ran out of colours to color Voronoi-tiles!")
         }
      });
   }
   println!("]");
   wheel
}

fn buidl_obj(protocol: &Book) -> String {
   let dyad = vol_24h_pair(&protocol);
   let ((prot, blok), valu) = unpair(&dyad);
   let v = valu.amount;
   let (protocol, block) = (quot(&prot), quot(&blok));
   let f2 = format!("protocol: {protocol}, blockchain: {block}");
   println!("\t{{ {f2}, value: {v} }},");
   block
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
   println!("*** REPLACE d.data.countries with d.data.protocol!");
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
