-- Your SQL goes here
CREATE TABLE users (
    uuid UUID NOT NULL PRIMARY KEY,
    username VARCHAR(64) NOT NULL,
    password VARCHAR(256) NOT NULL,
    auth_token CHAR(32) NOT NULL
);
