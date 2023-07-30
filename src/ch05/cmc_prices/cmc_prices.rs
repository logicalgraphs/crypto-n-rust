// given a filename, we parse the JSON and spit out the prices

// Easy-peasy!

use std::{
   env::var,
   io::Read
};

use crypto::{
   types::coins::print_all_coins,
   json_utils::parse_coins
};

fn main() -> Result<(), String> {
   let api_var = "COIN_MARKET_CAP_API_KEY";
   let api_key = var(api_var).expect(&format!("{api_var} not set!"));
   let cmc_endpoint = "https://pro-api.coinmarketcap.com/v1";
   let cmc_route = "cryptocurrency";
   let list_cmd = "listings/latest?start=1&limit=5000&convert=USD";

// formulating the Request header comes from the stackoverflow article:
// https://stackoverflow.com/questions/47911513/how-do-i-set-the-request-headers-using-reqwest

   let client = reqwest::Client::new();
   let mut res = client
      .get(&format!("{cmc_endpoint}/{cmc_route}/{list_cmd}"))
      .header("X-CMC_PRO_API_KEY", api_key)
      .header("Accept", "application/json")
      .send()
      .expect("Did not get a response from reqwest");

/* 
The skeleton upon which this get-fetch example is based is:

https://stackoverflow.com/questions/43222429/how-do-you-make-a-get-request-in-rust#:~:text=Sending%20a%20GET%20request%20is,send().unwrap()%3B%20assert_eq!
*/ 

   let mut body = String::new();
   res.read_to_string(&mut body).expect("Could not read fetch body");
   let coins = parse_coins(&body);
   print_all_coins(coins);
   Ok(())
}
