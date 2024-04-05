use tuples::TupleAsRef;
// https://stackoverflow.com/questions/75880055/rust-reference-of-tuple-and-tuple-of-reference

use book::{
   err_utils::ErrStr,
   num_utils::parse_or,
   utils::get_args
};

use crypto::{
   algos::orders::working_set,
   charts::venn::venn_diagram,
   types::books::load_books_from_stream
};

fn main() -> ErrStr<()> {
   let args = get_args();
   let mb_arg = args.first();
   let f = if let Some(n) = mb_arg.clone() {
      if n == "--help" { usage } else { thunk }
   } else { thunk };
   f(mb_arg)
}

fn thunk(mb_n: Option<&String>) -> ErrStr<()> {
   let books = load_books_from_stream()?;
   let default = 500.0;
   let min = parse_or(mb_n, default);
   let set = working_set(min, &books);
   println!("{}", venn_diagram(set.as_ref()));
   Ok(())
}

fn usage(_help: Option<&String>) -> ErrStr<()> {
   println!("echo <TSV file> | ./venn [min=500.0]\n");
   println!("\tOutputs a Venn diagram set, only allowing values above [min].");
   Ok(())
}
