CREATE TABLE IF NOT EXISTS topics
(
    topic_id              INTEGER             PRIMARY KEY NOT NULL,
    created_at            DATETIME            NOT NULL,
    topic_name            TEXT                NOT NULL,
    topic_description     TEXT                NOT NULL
);
