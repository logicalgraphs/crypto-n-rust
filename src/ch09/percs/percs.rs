use crypto::types::percentage::Percentage;

fn main() {
   let ps = vec!("5.7%","4%","-345.9%");
   for p in ps {
      let _p1 = print_percentage(p);
   }
}

fn print_percentage(p: &str) -> Result<Percentage, String> {
   let perc: Percentage = p.parse()?;
   println!("{p} is {perc}");
   Ok(perc)
}
