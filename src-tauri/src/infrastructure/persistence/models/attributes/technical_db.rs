use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TechnicalDatabase {
  pub vision: i64,
  pub passing: i64,
  pub heading: i64,
  pub crossing: i64,
  pub tackling: i64,
  pub dribbling: i64,
  pub finishing: i64,
}