use openfootball_lib::infrastructure::database_context::SqlitePersistenceContext;

fn setup_context() -> SqlitePersistenceContext {
  SqlitePersistenceContext::initialize_for_test()
    .expect("Failed to initialize persistence context for testing")
}

#[cfg(test)]
mod integration_tests {
  use super::*;
  use chrono::NaiveDate;
  use openfootball_lib::core::domain::entities::player::Player;
  use openfootball_lib::core::domain::enums::position::Position;
  use openfootball_lib::core::domain::repositories::player_repository::PlayerRepository;
  use openfootball_lib::core::domain::value_objects::mental_attributes::MentalAttributes;
  use openfootball_lib::core::domain::value_objects::physical_attributes::PhysicalAttributes;
  use openfootball_lib::core::domain::value_objects::technical_attributes::TechnicalAttributes;

  #[test]
  fn test_save_and_get_player() {
    let context = setup_context();
    let repository = &context.player_repository;

    let player_id = 123;
    let nation_id = 1;
    let birth_date = NaiveDate::parse_from_str("24-06-1987", "%d-%m-%Y").unwrap();
    let main_position = Position::RightWinger;

    let mut secondary_positions: Vec<Position> = Vec::new();
    secondary_positions.push(Position::LeftWinger);
    secondary_positions.push(Position::AttackingMidfielder);

    let mentals: MentalAttributes = {
      MentalAttributes { decision: 15, leadership: 15, positioning: 15, concentration: 15, determination: 15 }
    };

    let physicals: PhysicalAttributes = {
      PhysicalAttributes { pace: 15, stamina: 15, agility: 15, jumping: 15, strength: 15, acceleration: 15 }
    };

    let technicals: TechnicalAttributes = {
      TechnicalAttributes {  vision: 15, passing: 15, heading: 15, crossing: 15, tackling: 15, dribbling: 15, finishing: 15 } 
    };

    let original_player = Player::new(
      player_id, 
      nation_id, 
      "Messi", 
      "Lionel", 
      birth_date, 
      main_position, 
      secondary_positions, 
      mentals, 
      physicals, 
      technicals
    );

    repository.save(original_player.clone()).expect("Failed to save player");

    let retrieved_player = repository.get_by_id(
      i32::try_from(player_id)
      .expect("player_id out of i32 range")
    ).expect("Failed to get player");

    assert_eq!(retrieved_player.id, player_id);
    assert_eq!(retrieved_player.last_name, "Messi");
  }
}