-- Your SQL goes here
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    email VARCHAR NOT NULL,
    name VARCHAR NOT NULL,
    family_name VARCHAR NOT NULL,
    given_name VARCHAR NOT NULL,
    photo_url VARCHAR NOT NULL,
    uuid VARCHAR NOT NULL,
    experience_points BIGINT NOT NULL,
    score_history VARCHAR NOT NULL,
    app_difficulty_setting VARCHAR NOT NULL
)