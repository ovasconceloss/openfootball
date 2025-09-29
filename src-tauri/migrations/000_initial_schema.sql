CREATE TABLE players (
  id INTEGER PRIMARY KEY AUTOINCREMENT,
  nation_id INTEGER NOT NULL,
  last_name TEXT NOT NULL,
  first_name TEXT NOT NULL,
  birth_date TEXT NOT NULL,
  main_position TEXT NOT NULL,
  secondary_positions TEXT NOT NULL,
  mental_attributes TEXT NOT NULL,
  physical_attributes TEXT NOT NULL,
  technical_attributes TEXT NOT NULL
);