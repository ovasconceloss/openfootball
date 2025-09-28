use serde::{Deserialize, Serialize};
use crate::infrastructure::persistence::models::attributes::{
  mental_db::MentalDatabase, 
  physical_db::PhysicalDatabase, 
  technical_db::TechnicalDatabase
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerDatabase {
  pub id: i64,
  pub nation_id: i64,
  pub last_name: String,
  pub first_name: String,
  pub birth_date: String,
  pub main_position: String,
  pub secondary_positions: String,
  pub mental_attributes: MentalDatabase,
  pub physical_attributes: PhysicalDatabase,
  pub technical_attributes: TechnicalDatabase
}