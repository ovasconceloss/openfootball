use std::str::FromStr;
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct ParsePositionError;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Position {
  Goalkeeper,
  RightBack,
  LeftBack,
  CenterBack,

  DefensiveMidfielder,
  CentralMidfielder,
  AttackingMidfielder,
  RightMidfielder,
  LeftMidfielder,

  RightWinger,
  LeftWinger,
  Striker,
  SecondStriker
}

impl FromStr for Position {
  type Err = ParsePositionError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s.to_lowercase().as_str() {
      "goalkeeper" => Ok(Position::Goalkeeper),
      "rigthback" => Ok(Position::RightBack),
      "leftback" => Ok(Position::LeftBack),
      "centerback" => Ok(Position::CenterBack),
      "defensivemidfielder" => Ok(Position::DefensiveMidfielder),
      "centralmidfielder" => Ok(Position::CentralMidfielder),
      "attackingmidfielder" => Ok(Position::AttackingMidfielder),
      "rightmidfielder" => Ok(Position::RightMidfielder),
      "leftmidfielder" => Ok(Position::LeftMidfielder),
      "rightwinger" => Ok(Position::RightWinger),
      "leftwinger" => Ok(Position::LeftWinger),
      "striker" => Ok(Position::Striker),
      "secondstriker" => Ok(Position::SecondStriker),
      _ => Err(ParsePositionError),
    }
  }
}