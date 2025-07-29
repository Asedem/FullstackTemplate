CREATE DATABASE x;

\c x;

CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    user_name VARCHAR(32) NOT NULL UNIQUE
);

INSERT INTO users (user_name) VALUES ('Simon');
INSERT INTO users (user_name) VALUES ('Alice');
INSERT INTO users (user_name) VALUES ('Bob');
