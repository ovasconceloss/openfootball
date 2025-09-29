use serde::{Deserialize, Serialize};
use rusqlite::{types::ToSqlOutput, ToSql};
use crate::{
  core::domain::value_objects::physical_attributes::PhysicalAttributes, 
  infrastructure::persistence::models::errors::mapping_error::MappingError, 
};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct PhysicalDatabase {
  pub pace: i64,
  pub stamina: i64,
  pub agility: i64,
  pub jumping: i64,
  pub strength: i64,
  pub acceleration: i64,
}

impl ToSql for PhysicalDatabase {
  fn to_sql(&self) -> rusqlite::Result<ToSqlOutput<'_>> {
    let json_string = serde_json::to_string(self)
      .map_err(|error| rusqlite::Error::ToSqlConversionFailure(Box::new(error)))?;

    Ok(ToSqlOutput::Owned(rusqlite::types::Value::Text(json_string)))
  }
}

impl From<PhysicalAttributes> for PhysicalDatabase {
  fn from(physical_attributees: PhysicalAttributes) -> Self {
    PhysicalDatabase { 
      pace: i64::from(physical_attributees.pace), 
      stamina: i64::from(physical_attributees.stamina), 
      agility: i64::from(physical_attributees.agility), 
      jumping: i64::from(physical_attributees.jumping), 
      strength: i64::from(physical_attributees.strength), 
      acceleration: i64::from(physical_attributees.acceleration) 
    }
  }
}

impl TryFrom<PhysicalDatabase> for PhysicalAttributes {
  type Error = MappingError;

  fn try_from(physical_database: PhysicalDatabase) -> Result<Self, Self::Error> {
    Ok(PhysicalAttributes::new(
      physical_database.pace.try_into().unwrap(), 
      physical_database.stamina.try_into().unwrap(), 
      physical_database.agility.try_into().unwrap(), 
      physical_database.jumping.try_into().unwrap(), 
      physical_database.strength.try_into().unwrap(),
      physical_database.acceleration.try_into().unwrap()
    ))
  }
}