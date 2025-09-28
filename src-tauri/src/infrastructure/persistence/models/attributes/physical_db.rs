use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PhysicalDatabase {
  pub pace: i64,
  pub stamina: i64,
  pub agility: i64,
  pub jumping: i64,
  pub strength: i64,
  pub acceleration: i64,
}