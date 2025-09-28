use serde::{Deserialize, Serialize};
use crate::core::domain::value_objects::technical_attributes::TechnicalAttributes;

pub struct MappingError();

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

impl From<TechnicalAttributes> for TechnicalDatabase {
  fn from(technical_attributes: TechnicalAttributes) -> Self {
    TechnicalDatabase { 
      vision: i64::from(technical_attributes.vision), 
      passing: i64::from(technical_attributes.passing), 
      heading: i64::from(technical_attributes.heading), 
      crossing: i64::from(technical_attributes.crossing), 
      tackling: i64::from(technical_attributes.tackling), 
      dribbling: i64::from(technical_attributes.dribbling), 
      finishing: i64::from(technical_attributes.finishing) 
    }
  }
}

impl TryFrom<TechnicalDatabase> for TechnicalAttributes {
  type Error = MappingError;

  fn try_from(technical_database: TechnicalDatabase) -> Result<Self, Self::Error> {
    Ok(TechnicalAttributes::new(
      technical_database.vision.try_into().unwrap(), 
      technical_database.passing.try_into().unwrap(), 
      technical_database.heading.try_into().unwrap(), 
      technical_database.crossing.try_into().unwrap(), 
      technical_database.tackling.try_into().unwrap(),
      technical_database.dribbling.try_into().unwrap(),
      technical_database.finishing.try_into().unwrap()
    ))
  }
}