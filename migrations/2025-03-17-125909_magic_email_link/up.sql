-- Your SQL goes here
CREATE TABLE magiclinks (
  url TEXT PRIMARY KEY NOT NULL,
  ttl DATETIME NOT NULL,
  owned_by TEXT NOT NULL CHECK (owned_by = owned_by),
  created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
);
