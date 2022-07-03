-- Your SQL goes here
CREATE TABLE suggestions (
    suggestion_id INTEGER PRIMARY KEY ASC NOT NULL,
    suggestion_text TEXT NOT NULL,
    suggestion_date DATETIME NOT NULL,
    suggestion_author_id BIGINT NOT NULL,
    suggestion_message_id BIGINT NOT NULL
);