table! {
    refresh_tokens (token) {
        token -> Bytea,
        user_id -> Uuid,
        expires_at -> Timestamp,
        last_used_at -> Nullable<Timestamp>,
    }
}

table! {
    users (id) {
        id -> Uuid,
        username -> Varchar,
        password -> Text,
        blocked -> Bool,
        admin -> Bool,
    }
}

joinable!(refresh_tokens -> users (user_id));

allow_tables_to_appear_in_same_query!(refresh_tokens, users,);
