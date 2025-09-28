use serde::{Deserialize, Serialize};

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