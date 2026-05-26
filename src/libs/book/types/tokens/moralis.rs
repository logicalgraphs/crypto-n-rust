use crate::{ csv_utils::CsvWriter, currency::usd::{ USD, parse_usd } };

// Structs to parse the Moralis API response

#[derive(Deserialize, Debug)]
pub struct Tokens { result: Vec<TokenBalance> }

#[derive(Deserialize, Debug)]
pub struct TokenBalance {
    symbol: String,
    name: String,
    balance: String,
    decimals: Option<u8>,
    token_address: String,
    usd_price: USD,
    possible_spam: bool
}

impl TokenBalance {
   // Formats a raw string balance using the provided decimals
   pub fn bal(&self) -> String {
      let raw_balance = &self.balance;
      let deci = &self.decimals;
      match raw_balance.parse::<f64>() {
         Ok(val) => {
            let dec = deci.unwrap_or(18);
            let formatted = val / 10.0_f64.powi(dec as i32);
            format!("{:.4}", formatted)
            // Truncate to 4 decimal places for readability
         }
         Err(_) => raw_balance.to_string(),
      }
   }
}

