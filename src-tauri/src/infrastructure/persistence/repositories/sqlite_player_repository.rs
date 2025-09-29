use rusqlite::Connection;
use crate::{
  core::domain::{
    entities::player::Player, 
    enums::errors::repository_error::RepositoryError, 
    repositories::player_repository::PlayerRepository
  }, 
  infrastructure::persistence::models::{ 
    errors::mapping_error::MappingError, player_db::PlayerDatabase 
  }
};

const SQL_SAVE: &str = include_str!("../queries/player/save.sql");
const SQL_DELETE: &str = include_str!("../queries/player/delete.sql");
const SQL_GET_ALL: &str = include_str!("../queries/player/get_all.sql");
const SQL_GET_BY_ID: &str = include_str!("../queries/player/get_by_id.sql");
const SQL_GET_BY_CLUB: &str = include_str!("../queries/player/get_by_club.sql");

pub struct SqlitePlayerRepository {
  connection: Connection
}

impl SqlitePlayerRepository {
  pub fn new(database_path: &str) -> Result<Self, RepositoryError> {
    let connection = Connection::open(database_path)
      .map_err(|error| RepositoryError::DatabaseError(format!("Failed to open DB: {}", error)))?;

    Ok(SqlitePlayerRepository { connection })
  }
}

impl PlayerRepository for SqlitePlayerRepository {
  fn save(&self, player: Player) -> Result<(), RepositoryError> {
    let player_database = PlayerDatabase::from(player);

    let result = self.connection.execute(
      SQL_SAVE, 
      (
        player_database.id,
        player_database.nation_id,
        player_database.last_name,
        player_database.first_name,
        player_database.birth_date,
        player_database.main_position,
        player_database.secondary_positions,
        player_database.mental_attributes,
        player_database.physical_attributes,
        player_database.technical_attributes
      )
    );

    result
      .map(|_| ())
      .map_err(|error| RepositoryError::DatabaseError(error.to_string()))
  }

  fn delete(&self, id: i32) -> Result<(), RepositoryError> {
    let result = self.connection.execute(SQL_DELETE, [id]);

    result
      .map(|_| ())
      .map_err(|error| RepositoryError::DatabaseError(error.to_string()))
  }

  fn get_all(&self) -> Result<Vec<Player>, RepositoryError> {
    let mut statement = self.connection.prepare(SQL_GET_ALL)
      .map_err(|error| RepositoryError::DatabaseError(error.to_string()))?;

    let mental_idx = statement.column_index("mental_attributes")
    .map_err(|error| RepositoryError::DatabaseError(error.to_string()))?;

    let physical_idx = statement.column_index("physical_attributes")
    .map_err(|error| RepositoryError::DatabaseError(error.to_string()))?;

    let technical_idx = statement.column_index("technical_attributes")
    .map_err(|error| RepositoryError::DatabaseError(error.to_string()))?;

    let rows = statement.query_map([], |row| PlayerDatabase::from_row(row, mental_idx, physical_idx, technical_idx))
      .map_err(|error| RepositoryError::DatabaseError(error.to_string()))?;

    let players: Result<Vec<Player>, RepositoryError> = rows
      .map(|row_result| {
        let player_database = row_result
          .map_err(|error| RepositoryError::DatabaseError(error.to_string()))?;

        let player: Player = player_database.try_into()
          .map_err(|error: MappingError| RepositoryError::DatabaseError(error.0))?;

        Ok(player)
      }).collect();
    
    players
  }

  fn get_by_id(&self, id: i32) -> Result<Player, RepositoryError> {
    let mut statement = self.connection.prepare(SQL_GET_BY_ID)
      .map_err(|error| RepositoryError::DatabaseError(error.to_string()))?;

    let mental_idx = statement.column_index("mental_attributes")
      .map_err(|error| RepositoryError::DatabaseError(error.to_string()))?;

    let physical_idx = statement.column_index("physical_attributes")
      .map_err(|error| RepositoryError::DatabaseError(error.to_string()))?;

    let technical_idx = statement.column_index("technical_attributes")
      .map_err(|error| RepositoryError::DatabaseError(error.to_string()))?;

    let player_database = statement
      .query_row([id], |row| PlayerDatabase::from_row(row, mental_idx, physical_idx, technical_idx))
      .map_err(|error| match error {
        rusqlite::Error::QueryReturnedNoRows => RepositoryError::NotFound,
        unknow => RepositoryError::DatabaseError(unknow.to_string()),
      })?;

    let player: Player = player_database
      .try_into()
      .map_err(|error: MappingError| {
        eprintln!("{:?}", error);
        RepositoryError::DatabaseError(error.0)
      })?;

    Ok(player)
  }

  fn list_by_club(&self, id: i32) -> Result<Vec<Player>, RepositoryError> {
    let mut statement = self.connection.prepare(SQL_GET_BY_CLUB)
      .map_err(|error| RepositoryError::DatabaseError(error.to_string()))?;

    let mental_idx = statement.column_index("mental_attributes")
    .map_err(|error| RepositoryError::DatabaseError(error.to_string()))?;

    let physical_idx = statement.column_index("physical_attributes")
    .map_err(|error| RepositoryError::DatabaseError(error.to_string()))?;

    let technical_idx = statement.column_index("technical_attributes")
    .map_err(|error| RepositoryError::DatabaseError(error.to_string()))?;

    let rows = statement.query_map([id], |row| PlayerDatabase::from_row(row, mental_idx, physical_idx, technical_idx))
      .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

    let players: Result<Vec<Player>, RepositoryError> = rows
      .map(|row_result| {
        let player_database = row_result
          .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        let player: Player = player_database.try_into()
          .map_err(|error: MappingError| RepositoryError::DatabaseError(error.0))?;

        Ok(player)
      }).collect();
    
    players
  }
}