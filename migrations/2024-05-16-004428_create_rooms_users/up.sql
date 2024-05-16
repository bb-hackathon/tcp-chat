-- Your SQL goes here
CREATE TABLE rooms_users (
    room_uuid UUID REFERENCES rooms(uuid),
    user_uuid UUID REFERENCES users(uuid),
    PRIMARY KEY(room_uuid, user_uuid)
);
