-- Your SQL goes here
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    email VARCHAR NOT NULL,
    uuid VARCHAR NOT NULL,
    experience_points BIGINT NOT NULL,
    score_history VARCHAR NOT NULL
)