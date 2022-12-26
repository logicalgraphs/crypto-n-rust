use crate::types::{
   percentage::Percentage,
   usd::USD
};

#[derive(Debug, Clone)]
pub struct Liquidation {
   pub amount: USD,
   pub percentage: Percentage
}

impl Liquidation {
   pub fn weight(&self) -> f32 {
      self.percentage.of(self.amount.amount)
   }
}

pub fn mk_liquidation(amount: USD, percentage: Percentage) -> Liquidation {
   Liquidation { amount, percentage }
}

pub fn gather_liquidation_info(line: &mut Vec<&str>)
   -> Result<Option<Percentage>, String> {
   let ans = if line.first() == Some(&"LIQUIDATION") {
      line.pop().map(|p| p.parse().expect(&format!("percentage {p}")))
   } else {
      None
   };
   Ok(ans)
}
