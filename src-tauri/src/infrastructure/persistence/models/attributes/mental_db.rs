use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MentalDatabase {
  pub decision: i64,
  pub leadership: i64,
  pub positioning: i64,
  pub concentration: i64,
  pub determination: i64,
}