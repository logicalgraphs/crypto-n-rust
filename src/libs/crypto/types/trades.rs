// a trade is a swap from one asset to another

use crate::types::asset::Asset;

#[derive(Debug, Clone)]
pub struct Trade {
   date: String, // because this is relevant, how?
   from: Asset,
   to: Asset
}
