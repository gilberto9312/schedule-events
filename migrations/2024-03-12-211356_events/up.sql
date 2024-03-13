-- Your SQL goes here
CREATE TABLE events (
  id SERIAL PRIMARY KEY,
  title VARCHAR NOT NULL,
  date_start DATE NOT NULL,
  time_start TIME NOT NULL,
  time_end TIME NOT NULL,
  date_end DATE NOT NULL
)
