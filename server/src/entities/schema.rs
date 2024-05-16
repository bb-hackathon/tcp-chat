// @generated automatically by Diesel CLI.

diesel::table! {
    messages (uuid) {
        uuid -> Uuid,
        sender_uuid -> Uuid,
        room_uuid -> Uuid,
        text -> Text,
        timestamp -> Timestamp,
    }
}

diesel::table! {
    rooms (uuid) {
        uuid -> Uuid,
        #[max_length = 64]
        name -> Varchar,
    }
}

diesel::table! {
    rooms_users (room_uuid, user_uuid) {
        room_uuid -> Uuid,
        user_uuid -> Uuid,
    }
}

diesel::table! {
    users (uuid) {
        uuid -> Uuid,
        #[max_length = 64]
        username -> Varchar,
        #[max_length = 256]
        password -> Varchar,
        #[max_length = 32]
        auth_token -> Bpchar,
    }
}

diesel::joinable!(messages -> rooms (room_uuid));
diesel::joinable!(messages -> users (sender_uuid));
diesel::joinable!(rooms_users -> rooms (room_uuid));
diesel::joinable!(rooms_users -> users (user_uuid));

diesel::allow_tables_to_appear_in_same_query!(
    messages,
    rooms,
    rooms_users,
    users,
);
