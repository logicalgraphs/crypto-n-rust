use book::{
   num::percentage::Percentage,
   utils::{err_utils::ErrStr,env_utils::get_args}
};

fn main() -> ErrStr<()> {
   let args = get_args();
   if let Some(arg) = args.first() {
      let p: Percentage = arg.parse()?;
      println!("Answer is {p}");
      Ok(())
   } else {
      Err("Enter a percentage to parse".to_string())
   }
}
