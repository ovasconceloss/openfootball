#[derive(Clone, Debug)]
pub struct TechnicalAttributes {
  pub vision: i32,
  pub passing: i32,
  pub heading: i32,
  pub crossing: i32,
  pub tackling: i32,
  pub dribbling: i32,
  pub finishing: i32,
}

impl TechnicalAttributes {
  pub fn new(vision: i32, passing: i32, heading: i32, crossing: i32, tackling: i32, dribbling: i32, finishing: i32) -> Self {
    Self { 
      vision, 
      passing, 
      heading, 
      crossing,
      tackling,
      dribbling,
      finishing
    }
  }
}