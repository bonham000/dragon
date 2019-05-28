table! {
    users (id) {
        id -> Int4,
        email -> Varchar,
        name -> Varchar,
        family_name -> Varchar,
        given_name -> Varchar,
        photo_url -> Varchar,
        uuid -> Varchar,
        experience_points -> Int8,
        score_history -> Varchar,
        language_setting -> Varchar,
        app_difficulty_setting -> Varchar,
    }
}
