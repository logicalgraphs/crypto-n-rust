use book::{
   err_utils::ErrStr,
   currency::usd::USD,
   utils::get_args
};

fn main() -> ErrStr<()> {
   let args = get_args();
   if let Some(arg) = args.first() {
      let p: USD = arg.parse()?;
      println!("Answer is {p}");
      Ok(())
   } else {
      Err("Enter a dollar-amount to parse".to_string())
   }
}
