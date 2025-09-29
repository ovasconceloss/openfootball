use std::path::Path;
use rusqlite::Connection;
use crate::{
  core::domain::enums::errors::repository_error::RepositoryError, 
  infrastructure::persistence::{
    migrations::run_migrations, 
    repositories::sqlite_player_repository::SqlitePlayerRepository
  }
};

const IN_MEMORY_DB_PATH: &str = "./tests/openfootballv1.tests.db"; 

pub struct SqlitePersistenceContext {
  pub player_repository: SqlitePlayerRepository
}

impl SqlitePersistenceContext {
  pub fn initialize(database_path: &str) -> Result<Self, RepositoryError> {
    let mut connection = Connection::open(database_path)
      .map_err(|error| RepositoryError::DatabaseError(format!("DB connection failed: {}", error)))?;

    let migrations_directory = Path::new("./migrations");
    let _ = run_migrations(&mut connection, migrations_directory);

    let player_repository = SqlitePlayerRepository::new(database_path)?;

    Ok(SqlitePersistenceContext { 
      player_repository: player_repository 
    })
  }

  pub fn initialize_for_test() -> Result<Self, RepositoryError> {
    Self::initialize(IN_MEMORY_DB_PATH)
  }
}