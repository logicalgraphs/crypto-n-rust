use std::io::Read;

// The skeleton upon which this get-fetch example is based is:
// https://stackoverflow.com/questions/43222429/how-do-you-make-a-get-request-in-rust#:~:text=Sending%20a%20GET%20request%20is,send().unwrap()%3B%20assert_eq!

fn usage() {
   println!("./burn ntoks book1 book2 ... bookn");
   println!("\tTrades ntoks via the order books listed.");
   println!("\tA synthetic order book can be 1 bid with lots of tokens at");
   println!("\tthe ratio's quote.");
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
   let url = "https://api.kujira.app/api/coingecko/orderbook";
   let tick = "ticker_id=LOCAL_USK&depth=10";
   let res = reqwest::get(&format!("{url}?{tick}"));
   let mut body = String::new();
   res?.read_to_string(&mut body)?;
   reportage(&body);
   usage();
   Ok(())
}

fn reportage(body: &str) {
   println!("I got {body}");
}
