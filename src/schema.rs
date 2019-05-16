table! {
    messages (id) {
        id -> Int4,
        message -> Varchar,
        author -> Varchar,
        uuid -> Varchar,
    }
}

table! {
    users (id) {
        id -> Int4,
        email -> Varchar,
        uuid -> Varchar,
        experience_points -> Int8,
        score_history -> Varchar,
    }
}

allow_tables_to_appear_in_same_query!(
    messages,
    users,
);
