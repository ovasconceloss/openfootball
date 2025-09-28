use std::str::FromStr;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use crate::{
  core::domain::{
    entities::player::Player, 
    enums::position::Position, value_objects::{
      mental_attributes::MentalAttributes, 
      physical_attributes::PhysicalAttributes, 
      technical_attributes::TechnicalAttributes
    }
  }, 
  infrastructure::persistence::models::{attributes::{
    mental_db::MentalDatabase, 
    physical_db::PhysicalDatabase, 
    technical_db::TechnicalDatabase
  }, errors::mapping_error::MappingError}
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

impl From<Player> for PlayerDatabase {
  fn from(player: Player) -> Self {
    let birth_date_string = player.birth_date.format("%d-%m-%Y").to_string();

    let main_position_string = serde_json::to_string(&player.main_position).unwrap_or_default();
    let secondary_positions_string = serde_json::to_string(&player.secondary_positions).unwrap_or_default();

    let mental_database = MentalDatabase::from(player.mental_attributes);
    let physical_database = PhysicalDatabase::from(player.physical_attributes);
    let technical_database = TechnicalDatabase::from(player.technical_attributes);

    PlayerDatabase { 
      id: player.id, 
      nation_id: player.nation_id, 
      last_name: player.last_name, 
      first_name: player.first_name, 
      birth_date: birth_date_string, 
      main_position: main_position_string, 
      secondary_positions: secondary_positions_string, 
      mental_attributes: mental_database, 
      physical_attributes: physical_database, 
      technical_attributes: technical_database
    }
  }
}

impl TryFrom<PlayerDatabase> for Player {
  type Error = MappingError;

  fn try_from(player_database: PlayerDatabase) -> Result<Self, Self::Error> {
    let birth_date = NaiveDate::parse_from_str(&player_database.birth_date, "%d-%m-%Y")
      .map_err(|error| MappingError(format!("Invalid birth date format: {}", error)))?;

    let main_position = Position::from_str(&player_database.main_position)
      .map_err(|error| MappingError(format!("Invalid main position: {:?}", error)))?;

    let secondary_positions: Vec<Position> = serde_json::from_str(&player_database.secondary_positions)
      .map_err(|error| MappingError(format!("Invalid secondary positions JSON: {}", error)))?;

    let mental_attributes = MentalAttributes::try_from(player_database.mental_attributes)?;
    let physical_attributes = PhysicalAttributes::try_from(player_database.physical_attributes)?;
    let technical_attributes = TechnicalAttributes::try_from(player_database.technical_attributes)?;

    Ok(Player::new(
      player_database.id, 
      player_database.nation_id, 
      player_database.last_name, 
      player_database.first_name, 
      birth_date, 
      main_position, 
      secondary_positions, 
      mental_attributes, 
      physical_attributes, 
      technical_attributes 
    ))
  }
}