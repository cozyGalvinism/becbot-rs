-- Your SQL goes here
CREATE TABLE quotes (
    quote_id INTEGER PRIMARY KEY ASC NOT NULL,
    message TEXT NOT NULL,
    quote_author_id BIGINT NOT NULL,
    quote_author TEXT NOT NULL,
    date TIMESTAMP NOT NULL
);

CREATE TABLE cans (
    can_id INTEGER PRIMARY KEY ASC NOT NULL,
    user_id BIGINT NOT NULL,
    user TEXT NOT NULL,
    date TIMESTAMP NOT NULL
);