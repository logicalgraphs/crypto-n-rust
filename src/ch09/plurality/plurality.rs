use book::string_utils::plural;

fn main() {
   let trade = "trade";
   println!("I made {}.", plural(1, trade));
   println!("I made {}.", plural(0, trade));
   println!("I made {} yesterdæg, 'cuz I GOT BIZY, FAM!", plural(327, trade));

   let plur = "plural";
   let plurs: String = plural(2, plur);
   println!("\nBUT DID YOU KNOWWWWWWWW:

That the plural of '{plur}' is '{plurs}'?

Try it out! 😎 ");
}
