use std::marker::PhantomData;
use std::ops::Mul;

// 1. Define zero-sized marker units (they use 0 bytes of memory)
#[derive(Debug, Clone, Copy)] pub struct BTC;
#[derive(Debug, Clone, Copy)] pub struct ETH;
#[derive(Debug, Clone, Copy)] pub struct AVAX;
#[derive(Debug, Clone, Copy)] pub struct USDC;
// Add BNB, LINK, LTC, UNDEAD here...

// 2. Define a generic Crypto Amount using PhantomData
#[derive(Debug, Clone, Copy)]
pub struct Amount<Asset> {
    pub value: f64,
    _marker: PhantomData<Asset>,
}

// Helper function to create amounts cleanly
impl<Asset> Amount<Asset> {
    pub fn new(value: f64) -> Self {
        Self { value, _marker: PhantomData }
    }
}

// 3. Define the Quote price (Asset per USD)
#[derive(Debug, Clone, Copy)]
pub struct Quote<Asset> {
    pub price_usd: f64,
    _marker: PhantomData<Asset>,
}

impl<Asset> Quote<Asset> {
    pub fn new(price_usd: f64) -> Self {
        Self { price_usd, _marker: PhantomData }
    }
}

// 4. Define the single, final destination type
#[derive(Debug, Clone, Copy)]
pub struct TotalValue(pub f64);

// 5. Implement multiplication dynamically for ANY asset, but ONLY if they match!
// This reads: Multiply Amount<A> by Quote<A> where both 'A' are the exact same asset type.
impl<Asset> Mul<Quote<Asset>> for Amount<Asset> {
    type Output = TotalValue;

    fn mul(self, rhs: Quote<Asset>) -> Self::Output {
        TotalValue(self.value * rhs.price_usd)
    }
}

/*
fn main() {
    // Correctly matched types
    let btc_amount = Amount::<BTC>::new(1.5);
    let btc_quote = Quote::<BTC>::new(60000.0);
    
    let eth_amount = Amount::<ETH>::new(10.0);
    let eth_quote = Quote::<ETH>::new(30000.0);

    // This compiles successfully:
    let btc_total = btc_amount * btc_quote;
    let eth_total = eth_amount * eth_quote;
    println!("Valid math! BTC value: {}, ETH value: {}", btc_total.0, eth_total.0);

    // COMPILER ERROR: expected `Quote<ETH>`, found `Quote<BTC>`
    // let invalid_total = eth_amount * btc_quote; 
}
*/

