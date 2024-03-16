use tuples::TupleAsRef;
// https://stackoverflow.com/questions/75880055/rust-reference-of-tuple-and-tuple-of-reference

use book::err_utils::ErrStr;

use crypto::{
   algos::orders::working_set,
   charts::venn::venn_diagram,
   types::books::load_books_from_stream
};

fn main() -> ErrStr<()> {
   let books = load_books_from_stream()?;
   let set = working_set(500.0, &books);
   println!("{}", venn_diagram(set.as_ref()));
   Ok(())
}
