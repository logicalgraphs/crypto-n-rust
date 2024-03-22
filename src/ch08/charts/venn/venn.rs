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
   let books = load_books_from_stream()?;
   let min = parse_or(get_args().first(), 500.0);
   let set = working_set(min, &books);
   println!("{}", venn_diagram(set.as_ref()));
   Ok(())
}
