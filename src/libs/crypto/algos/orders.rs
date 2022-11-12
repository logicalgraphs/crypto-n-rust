use crate::types::{
   assets::Asset,
   marketplace::OrderBook,
};

pub fn target_sell_ratio(a: &Asset, on: &OrderBook, per: f32) -> Option<f32> {
   if on.ratio == 0.0 {
      None
   } else {
      Some(a.quote * per * on.ratio)
   }
}
