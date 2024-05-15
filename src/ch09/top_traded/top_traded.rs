use book::{
   list_utils::first_last,
   num_utils::parse_or,
   utils::get_args
};

use crypto::{
   algos::orders::working_set,
   charts::venn::venn_diagram,
   types::books::parse_books_with_aliases
};

use topper::reports::report;

fn usage() {
   println!("\n./top_traded <date> [min volume=50000]\n");
   println!("Prints the top-traded tokens by 24h-volumes.\n");
   println!("The set of sets can be represented as a Venn diagram using, i.e.");
   println!("https://github.com/benfred/venn.js");
}

fn main() {
   if let (Some(date), min) = first_last(&get_args()) {
      do_it(&date, min.as_ref());
   } else {
      usage();
   }
}

fn do_it(date: &str, min_opt: Option<&String>) {
   let (_, books) = parse_books_with_aliases(&date);
   let default_min: f32 = 50000.0;
   let min = parse_or(min_opt, default_min);
   let (vols, toks) = working_set(min, &books);
   println!("{}", venn_diagram((&vols, &toks)));
   report(date, vols);
}
