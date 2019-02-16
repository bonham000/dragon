-- Your SQL goes here
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    email VARCHAR,
    uuid VARCHAR NOT NULL,
    experience_points BIGINT NOT NULL,
    score_history VARCHAR NOT NULL
)