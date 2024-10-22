use book::{
   num::currency::usd::USD,
   utils::{
      env_utils::get_args,
      err_utils::ErrStr
   }
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
