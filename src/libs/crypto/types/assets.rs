#[derive(Debug, Clone)]
pub struct Asset {
   token: String,
   amount: f32,
   quote: f32
}

pub fn mk_asset(token: String, amount: f32, quote: f32) -> Asset {
   Asset { token, amount, quote }
}

pub fn parse_asset(tok: &str, amt: &str, quot: &str)
   -> Result<Asset, String> {
   let amount: f32 = amt.parse().expect("amount");
   let quote: f32 = quot.parse().expect("quote");
   let token = tok.to_string();
   Ok(Asset { token, amount, quote })
}

pub fn merge_assets(a1: Asset, a2: Asset) -> Asset {
   let token = a1.token;
   let amount = a1.amount + a2. amount;
   let quote = (a1.quote + a2.quote) / 2.0;
   Asset { token, amount, quote }
}

pub fn read_csv_asset(line: &String) -> Result<Asset, String> {
   if let [token, amount, quote] =
         line.split(',').collect::<Vec<&str>>().as_slice() {
      parse_asset(token, amount, quote)
   } else {
      Err("Can't parse line: ".to_owned() + line)
   }
}
