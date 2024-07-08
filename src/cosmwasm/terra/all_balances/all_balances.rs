use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use tokio;

use book::{
   err_utils::{ErrStr,err_or},
   num_utils::parse_num,
   utils::get_env
};

#[derive(Serialize, Deserialize, Debug)]
struct BalanceResponse {
   balances: Vec<Coin0>
}

#[derive(Serialize, Deserialize, Debug)]
struct Coin0 {
   denom: String,
   amount: String
}

#[derive(Clone, Debug)]
struct Coin {
   token: String,
   amount: f32
}

fn denoms(s: &str) -> String {
   let toks0 =
      vec![("ibc/52B30BB501A222D586222700F241EBC8CA97A4A17C9737DDCC00DD0BBC24CEAD", "ASTROPEPE"),
           ("ibc/8D8A7F7253615E5F76CB6252A1E1BD921D5EDB7BBAAF8913FB1C77FF125D9995", "ASTRO"),
           ("ibc/B3F639855EE7478750CC8F82072307ED6E131A8EFF20345E1D136B50C4E5EC36", "ampWHALE"),
           ("uluna", "LUNA")];
   let toks: HashMap<String, String> =
       toks0.into_iter().map(|(a,b)| (a.to_string(), b.to_string())).collect();
   toks.get(s).expect(&format!("No value for key {s}")).to_string()
}

fn mk_coin(c: &Coin0) -> Coin {
   let amount = parse_num(&c.amount).expect(&format!("{} is NaN", c.amount));
   let token = denoms(&c.denom);
   Coin { token, amount }
}

fn print_coin(c0: &Coin0) -> String {
   let c: Coin = mk_coin(c0);
   format!("Coin:\n  token: {}\n  amount: {:?}", c.token, c.amount / 1000000.0)
}

#[tokio::main]
async fn main() -> ErrStr<()> {
   let wallet_addy = get_env("TERRA_ADDY")?;
   let lcd = "https://lcd-terra.tfl.foundation";
   let swagger_url = format!("{lcd}/cosmos/bank/v1beta1/balances");
   let url = format!("{swagger_url}/{wallet_addy}");

   let get = err_or(reqwest::get(url).await, "GET")?;
   let response = err_or(get
       .json::<BalanceResponse>()
       .await, "bad response")?;

   let res: Vec<String> = response.balances.iter().map(print_coin).collect();
   println!("\nBalances:\n\n{}", res.join("\n\n"));

   Ok(())
}
