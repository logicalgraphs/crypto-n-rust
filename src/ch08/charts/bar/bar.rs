// Creates a bar-chart from TSV-data.

// "What TSV-data?" you ask.
// currently, I'm charting protocols and ROI.
// That TVS-data.

use book::{
   file_utils::extract_date_and_body,
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
         let (date, body) = extract_date_and_body(&file);
         let protocols = tail(body);
         buidl_arr(&protocols);
         draw_bar_chart(&date);
      }
   } else {
      usage();
   }
}

fn print_prelude() {
   let obs = "https://observablehq.com/@observablehq";
   let bar = "plot-state-population-change?intent=fork";
   println!("goto:\n{obs}/{bar}\n");
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

fn draw_bar_chart(date: &str) {
   let sp = format!("+ {}",  quot(" "));
   println!("label = d => d.blockchain{sp} + d.protocol");
   let top = quot("top");
   let prot = quot(&format!("Protocol ROI, {date}"));
   let ctr = quot("center");
   let plus = quot("+");
   let piggy = quot("PiYg");
   let ordi = quot("ordinal");
   let x = quot("x");
   let whit = quot("white");
   println!("Plot.plot({{
  label: null,
  x: {{
    axis: {top},
    label: {prot},
    labelAnchor: {ctr},
    tickFormat: {plus},
    percent: true
  }},
  color: {{
    scheme: {piggy},
    type: {ordi}
  }},
  marks: [
    Plot.barX(protocols, {{
       y: label, 
       x: (d) => d.gain, 
       fill: (d) => Math.sign(d.gain), 
       sort: {{y: {x}}}}}),
    Plot.gridX({{stroke: {whit}, strokeOpacity: 0.5}}),
    Plot.axisY({{x: 0}}),
    Plot.ruleX([0])
  ]
}})");
}
