use crate::core::domain::{
  entities::player::Player, 
  enums::errors::repository_error::RepositoryError
};

pub trait PlayerRepository {
  fn delete(&self, id: i32) -> anyhow::Result<(), RepositoryError>;
  fn get_all(&self) -> anyhow::Result<Vec<Player>, RepositoryError>;
  fn save(&self, player: Player) -> anyhow::Result<(), RepositoryError>;
  fn get_by_id(&self, id: i32) -> anyhow::Result<Player, RepositoryError>;
  fn list_by_club(&self, club_id: i32) -> anyhow::Result<Vec<Player>, RepositoryError>;
}