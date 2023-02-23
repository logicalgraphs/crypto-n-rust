// Creates a bar-chart from TSV-data.

// "What TSV-data?" you ask.
// currently, I'm charting protocols and ROI.
// That TVS-data.

use book::{
   file_utils::lines_from_file,
   list_utils::tail,
   utils::get_args
};

use crypto::types::percentage::Percentage;

fn usage() {
   println!("./bar <tsv-file>");
   println!("\n\trenders a bar-chart of protocol and ROI-data");
}

fn main() {
   let files = get_args();
   if !files.is_empty() {
      print_prelude();
      for file in files {
         let protocols = tail(lines_from_file(&file));
         buidl_arr(&protocols);
         draw_bar_chart();
      }
   } else {
      usage();
   }
}

fn print_prelude() {
   let url = "https://observablehq.com/@observablehq/plot-bar?collection=@observablehq/plot";
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
   if let [prot, blok, _, _, gan] = things.as_slice() {
      let gain: Percentage =
         gan.parse().expect(&format!("Not a percentage: {gan}"));
      let per = gain.percent;
      let (protocol, block) = (quot(prot), quot(blok));
      let f2 = format!("protocol: {protocol}, blockchain: {block}");
      println!("\t{{ {f2}, gain: {per} }},");
   } else {
      println!("Could not parse line: {protocol}");
   }
}

fn draw_bar_chart() {
   let blok = quot("blockchain");

   println!("label = d => d.protocol + \" \" + d.blockchain[0]\n");
   println!("Plot.plot({{
  x: {{
    domain: d3.sort(protocols, d => -d.gain).map(label)
  }},
  y: {{
    grid: true
  }},
  marks: [
    Plot.barY(protocols, {{x: label, y: d => d.gain * 100, fill: {blok}}}),
    Plot.ruleY([0])
  ]
}})");
}
