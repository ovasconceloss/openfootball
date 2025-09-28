use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use crate::core::domain::{
  enums::position::Position, 
  value_objects::{
    mental_attributes::MentalAttributes, 
    physical_attributes::PhysicalAttributes, 
    technical_attributes::TechnicalAttributes
  }
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Player {
  pub id: i64,
  pub nation_id: i64,
  pub last_name: String,
  pub first_name: String,
  pub birth_date: NaiveDate,
  pub main_position: Position,
  pub secondary_positions: Vec<Position>,
  pub mental_attributes: MentalAttributes,
  pub physical_attributes: PhysicalAttributes,
  pub technical_attributes: TechnicalAttributes,
}

impl Player {
  pub fn new(
    id: i64, nation_id: i64, last_name: impl Into<String>, first_name: impl Into<String>, 
    birth_date: NaiveDate, main_position: Position, secondary_positions: Vec<Position>, 
    mentals: MentalAttributes, physicals: PhysicalAttributes, technicals: TechnicalAttributes
  ) -> Self {
    Self { 
      id, 
      nation_id, 
      last_name: last_name.into(),
      first_name: first_name.into(),
      birth_date,
      main_position,
      secondary_positions: secondary_positions,
      mental_attributes: mentals,
      physical_attributes: physicals,
      technical_attributes: technicals
    }
  }
}