-- Your SQL goes here
CREATE TABLE urls (
  url TEXT PRIMARY KEY NOT NULL,
  destination_url TEXT NOT NULL,
  ttl DATETIME,
  owned_by TEXT,
  created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
  updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

CREATE TRIGGER urls_update_at_trigger AFTER
UPDATE On urls BEGIN
UPDATE urls
SET
  updated_at = STRFTIME ('%Y-%m-%d %H:%M:%f', 'NOW')
WHERE
  URLS = NEW.URLS;

END;
