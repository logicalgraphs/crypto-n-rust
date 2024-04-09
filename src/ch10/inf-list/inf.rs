use book::list_utils::mk_inf;

fn main() {
   let ones = mk_inf(&[].to_vec(), 1);
   let count = mk_inf(&[1,2,3].to_vec(), 4);
   let mucho = mk_inf(&String::from("I love you this mucho").into_bytes(),
                     'o' as u8);
   let mondo = String::from_utf8(mucho.iter().take(70).collect()).unwrap();

   println!("ones are {ones:?}");
   println!("count is {count:?}");
   println!("How much do I love you?\n{mondo}...!");
}
