-- Add migration -- 0001_init.sql
CREATE TABLE IF NOT EXISTS todos (
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    text        TEXT NOT NULL,
    done        INTEGER NOT NULL DEFAULT 0,   -- 0 = false, 1 = true
    created_at  DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
);
