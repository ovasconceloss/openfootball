#[derive(Debug)]
pub enum RepositoryError {
  NotFound,
  DatabaseError(String)
}