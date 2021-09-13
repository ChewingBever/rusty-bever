table! {
    posts (id) {
        id -> Uuid,
        section_id -> Uuid,
        title -> Nullable<Varchar>,
        publish_date -> Date,
        content -> Text,
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
    sections (id) {
        id -> Uuid,
        title -> Varchar,
        description -> Nullable<Text>,
        is_default -> Bool,
        has_titles -> Bool,
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

joinable!(posts -> sections (section_id));
joinable!(refresh_tokens -> users (user_id));

allow_tables_to_appear_in_same_query!(posts, refresh_tokens, sections, users,);
