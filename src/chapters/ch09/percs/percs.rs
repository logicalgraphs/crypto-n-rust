use crypto::types::percentage::Percentage;

fn main() {
   let ps = vec!("5.7%","4%","-345.9%");
   for p in ps {
      let _p1 = print_percentage(p);
   }

   // Bonus questions:

   // What's 6% of 20?
   // What's 5% of 100?
   // What's 87% of 12?

   ["6%", "5%", "87%"]
      .iter().map(|p| p.parse().expect("not a percentage"))
             .zip([20.0, 100.0, 12.0])
             .for_each(whats);
}

fn print_percentage(p: &str) -> Result<Percentage, String> {
   let perc: Percentage = p.parse()?;
   println!("{p} is {perc}");
   Ok(perc)
}

fn whats((p, amt): (Percentage, f32)) {
   println!("{p} of {amt} is {}", p.of(amt));
}
