-- Your SQL goes here
CREATE TABLE messages (
    uuid UUID NOT NULL PRIMARY KEY,
    sender_uuid UUID NOT NULL REFERENCES users(uuid),
    room_uuid UUID NOT NULL REFERENCES rooms(uuid),
    text TEXT NOT NULL,
    timestamp TIMESTAMP NOT NULL
);
