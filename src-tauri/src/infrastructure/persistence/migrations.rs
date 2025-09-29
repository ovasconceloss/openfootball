use std::fs;
use std::path::Path;
use rusqlite::{params, Connection, OptionalExtension};
use crate::core::domain::enums::errors::repository_error::RepositoryError;

const SQL_SELECT: &str = include_str!("./queries/migrations/select.sql");
const SQL_INSERT: &str = include_str!("./queries/migrations/insert.sql");
const SQL_CREATE_TABLE: &str = include_str!("./queries/migrations/create_table.sql");

pub fn run_migrations(connection: &mut Connection, migrations_directory: &Path) -> Result<(), RepositoryError> {
  connection.execute_batch(SQL_CREATE_TABLE).map_err(|error| RepositoryError::DatabaseError(error.to_string()))?;
  connection.execute_batch("BEGIN EXCLUSIVE;").map_err(|error| RepositoryError::DatabaseError(error.to_string()))?;

  let mut entries: Vec<_> = fs::read_dir(migrations_directory)
    .map_err(|error| RepositoryError::DatabaseError(error.to_string()))?
    .filter_map(|e| e.ok())
    .filter(|entry| {
      let name = entry.file_name().to_string_lossy().to_string();
      name.ends_with(".sql")
    }).collect();

  entries.sort_by_key(|entry| entry.file_name());

  for entry in entries {
    let path = entry.path();
    let filename = entry.file_name().to_string_lossy().to_string();

    let mut statement = connection.prepare(SQL_SELECT)
      .map_err(|error| RepositoryError::DatabaseError(error.to_string()))?;

    let already: Option<i32> = statement.query_row([&filename], |row| row.get(0)).optional()
      .map_err(|error| RepositoryError::DatabaseError(error.to_string()))?;

    if already.is_some() { continue; }

    let sql_content = fs::read_to_string(&path)
      .map_err(|error| RepositoryError::DatabaseError(error.to_string()))?;

    connection.execute_batch("BEGIN;").map_err(|error| RepositoryError::DatabaseError(error.to_string()))?;
    let apply_result = (|| {
      connection.execute_batch(&sql_content)
        .map_err(|error| RepositoryError::DatabaseError(error.to_string()))?;

      connection.execute(SQL_INSERT, params![filename])
        .map_err(|error| RepositoryError::DatabaseError(error.to_string()))?;

      Ok::<(), RepositoryError>(())
    })();

    match apply_result {
      Ok(_) => {
        connection.execute_batch("COMMIT;")
          .map_err(|error| RepositoryError::DatabaseError(error.to_string()))?;
      }
      Err(error) => {
        let _ = connection.execute_batch("ROLLBACK;");
        let _ = connection.execute_batch("COMMIT;");

        return Err(error);
      }
    }
  }
  
  connection.execute_batch("COMMIT;").map_err(|error| RepositoryError::DatabaseError(error.to_string()))?;

  Ok(())
}