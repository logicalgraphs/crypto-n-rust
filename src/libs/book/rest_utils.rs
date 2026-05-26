use reqwest::{ Response, header::{ HeaderMap, HeaderValue } };
use serde::Deserialize;

use super::err_utils::{ErrStr,err_or};

/*
The skeleton upon which this get-fetch example is based is:

https://stackoverflow.com/questions/43222429/how-do-you-make-a-get-request-in-rust#:~:text=Sending%20a%20GET%20request%20is,send().unwrap()%3B%20assert_eq!

include:

reqwest = "0.9.18"

in the Cargo.toml-build-man&ifest
*/

/// a simple REST request-response
pub async fn read_rest(endpoint: &str) -> ErrStr<String> {
   let res = err_or(reqwest::get(endpoint).await, "https::GET")?;
   handle_response(res).await
}

/// When we need to send a REST request with headers
pub async fn read_rest_with(hm: HeaderMap, url: &str) -> ErrStr<String> {
   let client = reqwest::Client::new();
   let response = err_or(client
            .get(url)
            .headers(hm.clone())
            .send()
            .await, 
      &format!("Could not get a response from {url} with headers {hm:?}"))?;
   handle_response(response).await
}

async fn handle_response(response: Response) -> ErrStr<String> {
   if response.status().is_success() {
      let body = err_or(response.text().await, "no text in response")?;
      Ok(body)
   } else {
      let status = response.status();
      let error_body = err_or(response.text().await, "no error in text")?;
      Err(format!("Error status: {status}; Error body: {error_body}"))
   }
}

// ----- WALLETS -----------------------------------------------------

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
    usd_price: f32,
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

// Function to fetch native balance (e.g., ETH, MATIC)
pub async fn fetch_wallet_balances(chain: &str, address: &str, api_key: &str)
      -> ErrStr<Tokens> {

/*
This function models the following cURL command:

curl --request GET \
  --url 'https://deep-index.moralis.io/api/v2.2/wallets/{address}/tokens?chain=eth' \
  --header 'X-API-Key: test'
*/

    let url0 = "https://deep-index.moralis.io/api/v2.2/wallets";
    let url = format!("{url0}/{address}/tokens?chain={chain}");
    let mut headers = HeaderMap::new();
    let api_hdr = err_or(HeaderValue::from_str(api_key),
                         "Cannot insert MORALIS_API_KEY into header")?;
    headers.insert("X-API-Key", api_hdr);

    let client = reqwest::Client::new();
    let res = 
       err_or(client.get(&url).headers(headers).send().await,
              "Failed to send reqwest to moralis.io")?;
    let toks: Tokens =
       err_or(res.json().await, "Cannot convert response from JSON")?;
    Ok(toks)
}

// ----- TESTS -------------------------------------------------------

#[cfg(test)]
#[cfg(not(tarpaulin_include))]
mod sample_url {
   fn git_lg_url() -> String {
      "https://raw.githubusercontent.com/logicalgraphs".to_string()
   }

   fn rez(dir: &str, branch: &str, res: &str) -> String {
      format!("{}/crypto-n-rust/{branch}/data-files/{dir}/{res}", git_lg_url())
   }

   pub fn data_res(branch: &str, res: &str) -> String {
      rez("csv", branch, res)
   }

   pub fn quotes() -> String { data_res("main", "quotes.csv") }
}

#[cfg(test)]
#[cfg(not(tarpaulin_include))]
mod functional_tests {
   use std::{ cmp::Eq, collections::HashSet, hash::Hash };
   use super::*;
   use super::sample_url::quotes;
   use paste::paste;
   use crate::{
      create_testing,
      currency::usd::{ mk_usd, no_monay },
      num_utils::parse_num,
      string_utils::words,
      utils::{ get_env, now }
   };

   create_testing!("rest_utils");
   run!("read_rest", {
      let qts = now(read_rest(&quotes()))?;
      println!("\tQuotes from the LogicalGraphs REST endpoint:\n\n{}",
               qts.chars().take(1000).collect::<String>());
   });

   fn wallet_info(wallet: &str) -> ErrStr<(String, String, Vec<String>)> {
      let api_key = get_env("MORALIS_API_KEY")?;
      let wallet_address = get_env(wallet)?;

      // Chains supported by Moralis API
      let chains = words("eth bsc avalanche");
      // also: polygon arbitrum optimism fantom
      Ok((api_key, wallet_address, chains))
   }

   trait Container<T> { fn contains(&self, elt: &T) -> bool; }
   struct MyHashSet<T> {
      set: HashSet<T>
   }
   impl<T: Eq + Hash> Container<T> for MyHashSet<T> {
      fn contains(&self, t: &T) -> bool { self.set.contains(t) }
   }
   struct Yes90125;
   impl<T> Container<T> for Yes90125 {
      fn contains(&self, _t: &T) -> bool { true }
   }

   run!("fetch_wallet_balances_whitelisted", {
               let whitelist: HashSet<String> = words("
Protocol
0xeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeeee
Binance
vasBNB 0xcc1db43a06d97f736c7b045aedd03c6707c09bdf
DOGE 0xba2ae424d960c26247dd6c32edc70b295c744c43
LTC 0x4338665cbb7b2485a8855a139b75d5e34ab0db94
vBNB 0xa07c5b74c9b40447a954e1466938b865b6bbea36
LINK 0xf8a0bf9cf54bb92f17374d9e9a321e6a111a51bd
ETH 0x2170ed0880ac9a755fd29b2688956bd959f933f8
vWBETH 0x6cfdec747f37daf3b87a35a1d9c8ad3063a1a8a0
BTCB 0x7130d2a12b9bcbfae4f2634d864a1ee1ce3ead9c
asBNB 0x77734e70b6e88b4d82fe632a168edf6e700912b6
Avalanche
qiETH 0x334ad834cd4481bb02d09615e7c11a00579a7909
UNDEAD 0x5a3534720a4f29fa0dc53ce474db88973a95f65c
USDt 0x9702230a8ea53601f5cd2dc00fdbc13d4df4a8c7
USDC 0xb97ef9ef8734c71904d8002f8b6bc66dd9c48a6e").into_iter().collect();
      let mines = MyHashSet { set: whitelist };
      now(iter_chains_on(mines))
   });

   run!("fetch_wallet_balances_no_filter", {
      let alles = Yes90125;
      now(iter_chains_on(alles))
   });

   async fn iter_chains_on(whitelist: impl Container<String>) -> ErrStr<()> {
      let (api_key, wallet_address, chains) = wallet_info("WALLET_ADDY")?;
      for chain in chains {
         println!("\n=== Chain: {} ===", chain.to_uppercase());

         match fetch_wallet_balances(&chain, &wallet_address, &api_key).await {
            Ok(toks) => {
               let tokens = &toks.result;
               if tokens.is_empty() {
                  println!("No tokens found.");
               } else {
                  println!("Name,Symbol,Price,Amount,Total");
                  let mut total = no_monay();
                  for token in tokens {
                     
                     if whitelist.contains(&token.token_address) {
                        let boo = token.bal();
                        let bal = parse_num(&boo)?;
                        let quote = mk_usd(token.usd_price);
                        let totes = mk_usd(quote.amount * bal);
                        println!("{},{},{},{},{}", token.name, token.symbol,
                                 quote, boo, totes);
                        total += totes;
                     }
                  }
                  println!("\nTotal: {total}");
               }
            }
            Err(e) => println!("Failed to fetch ERC20 balances: {}", e),
         }
      }
      Ok(())
   }
}

#[cfg(not(tarpaulin_include))]
#[cfg(test)]
mod tests {
   use super::*;
   use super::sample_url::{quotes,data_res};

   #[tokio::test] async fn test_read_rest_ok() {
      let ans = read_rest(&quotes()).await;
      assert!(ans.is_ok());
   }

   #[tokio::test] async fn test_read_rest_err() {
      let ans = read_rest(&data_res("main", "schmivits.csv")).await;
      assert!(ans.is_err());
   }

   #[tokio::test] async fn test_read_rest_lines() -> ErrStr<()> {
      let ans = read_rest(&quotes()).await?;
      assert!(ans.len() > 5);
      Ok(())
   }
}

