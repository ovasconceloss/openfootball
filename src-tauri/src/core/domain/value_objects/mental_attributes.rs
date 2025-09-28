use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MentalAttributes {
  pub decision: i32,
  pub leadership: i32,
  pub positioning: i32,
  pub concentration: i32,
  pub determination: i32,
}

impl MentalAttributes {
  pub fn new(decision: i32, leadership: i32, positioning: i32, concentration: i32, determination: i32) -> Self {
    Self { 
      decision, 
      leadership, 
      positioning, 
      concentration, 
      determination 
    }
  }
}