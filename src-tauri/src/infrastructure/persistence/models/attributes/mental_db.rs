use serde::{Deserialize, Serialize};
use crate::{
  core::domain::value_objects::mental_attributes::MentalAttributes, 
  infrastructure::persistence::models::errors::mapping_error::MappingError,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MentalDatabase {
  pub decision: i64,
  pub leadership: i64,
  pub positioning: i64,
  pub concentration: i64,
  pub determination: i64,
}

impl From<MentalAttributes> for MentalDatabase {
  fn from(mental_attributes: MentalAttributes) -> Self {
    MentalDatabase { 
      decision: i64::from(mental_attributes.decision), 
      leadership: i64::from(mental_attributes.leadership), 
      positioning: i64::from(mental_attributes.positioning), 
      concentration: i64::from(mental_attributes.concentration), 
      determination: i64::from(mental_attributes.determination) 
    } 
  }
}

impl TryFrom<MentalDatabase> for MentalAttributes {
  type Error = MappingError;

  fn try_from(mental_database: MentalDatabase) -> Result<Self, Self::Error> {
    Ok(MentalAttributes::new(
      mental_database.decision.try_into().unwrap(), 
      mental_database.leadership.try_into().unwrap(), 
      mental_database.positioning.try_into().unwrap(), 
      mental_database.concentration.try_into().unwrap(), 
      mental_database.determination.try_into().unwrap()
    ))
  }
}