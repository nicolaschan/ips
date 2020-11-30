-- Your SQL goes here
CREATE TABLE hits (
  id SERIAL PRIMARY KEY NOT NULL,
  ip_addr TEXT NOT NULL,
  timestamp TIMESTAMP NOT NULL
)
