use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PhysicalAttributes {
  pub pace: i32,
  pub stamina: i32,
  pub agility: i32,
  pub jumping: i32,
  pub strength: i32,
  pub acceleration: i32,
}

impl PhysicalAttributes {
  pub fn new(pace: i32, stamina: i32, agility: i32, jumping: i32, strength: i32, acceleration: i32) -> Self {
    Self { 
      pace, 
      stamina, 
      agility, 
      jumping,
      strength,
      acceleration 
    }
  }
}