-- Your SQL goes here
CREATE TABLE IF NOT EXISTS tweets
(
    id          UUID PRIMARY KEY        NOT NULL,
    created_at  TIMESTAMP DEFAULT now() NOT NULL,
    messages    TEXT                    NOT NULL
    );