table! {
    permissions (id) {
        id -> Uuid,
        user_id -> Uuid,
        name -> Varchar,
    }
}

table! {
    refresh_tokens (token) {
        token -> Bytea,
        user_id -> Uuid,
        expires_at -> Timestamp,
        last_used_at -> Nullable<Timestamp>,
    }
}

table! {
    security_reports (id) {
        id -> Uuid,
        report_time -> Timestamp,
        report_type -> Varchar,
        content -> Text,
    }
}

table! {
    users (id) {
        id -> Uuid,
        username -> Varchar,
        password -> Text,
        blocked -> Bool,
    }
}

joinable!(permissions -> users (user_id));
joinable!(refresh_tokens -> users (user_id));

allow_tables_to_appear_in_same_query!(
    permissions,
    refresh_tokens,
    security_reports,
    users,
);
