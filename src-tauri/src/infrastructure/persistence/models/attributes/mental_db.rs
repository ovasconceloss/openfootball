use serde::{Deserialize, Serialize};
use rusqlite::{types::ToSqlOutput, ToSql};
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

impl ToSql for MentalDatabase {
  fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
    let json_string = serde_json::to_string(self)
      .map_err(|error| rusqlite::Error::ToSqlConversionFailure(Box::new(error)))?;

    Ok(ToSqlOutput::Owned(rusqlite::types::Value::Text(json_string)))
  }
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