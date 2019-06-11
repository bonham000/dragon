use ::uuid::Uuid;
use diesel;
use diesel::prelude::*;
use serde_json;

use super::schema::users::dsl::*;
use super::types::{InsertableUser, ListScore, InitialUserData, SavedUser, ScoreHistory, UserSettings};

pub fn get_user(
    user_uuid: String,
    connection: &PgConnection,
) -> Result<SavedUser, String> {
    println!("Fetching existing user");
    let result = find_user_by_uuid(&user_uuid, connection);
    match result {
        Ok(user) => Ok(user),
        Err(_) => Err("Error fetching user!".to_string()),
    }
}

pub fn create_user(
    user: InitialUserData,
    connection: &PgConnection,
) -> Result<SavedUser, String> {
    println!("Creating new user");
    let user = create_new_user(user);
    let result = insert_new_user(user, connection);
    match result {
        Ok(user) => Ok(user),
        Err(_) => Err("Something bad happened!".to_string()),
    }
}

pub fn update_user(user: SavedUser, connection: &PgConnection) -> QueryResult<SavedUser> {
    let user_uuid: String = user.uuid;
    let user_experience: i64 = user.experience_points;
    let scores: String = user.score_history;
    let user_settings: String = user.settings;
    let user_push_token: String = user.push_token;

    diesel::update(users.filter(uuid.eq(user_uuid)))
        .set((
            score_history.eq(scores),
            settings.eq(user_settings),
            push_token.eq(user_push_token),
            experience_points.eq(user_experience),
        ))
        .get_result(connection)
}

pub fn delete_user(user_uuid: String, connection: &PgConnection) -> QueryResult<usize> {
    diesel::delete(users.filter(uuid.eq(user_uuid))).execute(connection)
}

fn insert_new_user(user: InsertableUser, connection: &PgConnection) -> QueryResult<SavedUser> {
    diesel::insert_into(users)
        .values(&user)
        .get_result(connection)
}

fn create_new_user(user: InitialUserData) -> InsertableUser {
    let default_score_history = ScoreHistory {
        mc_english: false,
        mc_mandarin: false,
        quiz_text: false,
        mandarin_pronunciation: false,
        list_02_score: ListScore {
            complete: false,
            list_index: 0,
            list_key: "2".to_string(),
            number_words_completed: 0,
        },
        list_03_score: ListScore {
            complete: false,
            list_index: 1,
            list_key: "3".to_string(),
            number_words_completed: 0,
        },
        list_04_score: ListScore {
            complete: false,
            list_index: 2,
            list_key: "4".to_string(),
            number_words_completed: 0,
        },
        list_05_score: ListScore {
            complete: false,
            list_index: 3,
            list_key: "5".to_string(),
            number_words_completed: 0,
        },
        list_06_score: ListScore {
            complete: false,
            list_index: 4,
            list_key: "6".to_string(),
            number_words_completed: 0,
        },
    };

    let default_settings = UserSettings {
        disable_audio: false,
        auto_proceed_question: true,
        language_setting: "simplified".to_string(),
        app_difficulty_setting: "MEDIUM".to_string(),
    };

    InsertableUser {
        uuid: Uuid::new_v4().to_string(),
        email: "".to_string(),
        username: "".to_string(),
        experience_points: 0,
        push_token: user.push_token,
        settings: serde_json::to_string(&default_settings).unwrap(),
        score_history: serde_json::to_string(&default_score_history).unwrap(),
    }
}

fn find_user_by_uuid(user_uuid: &str, connection: &PgConnection) -> QueryResult<SavedUser> {
    users.filter(uuid.eq(user_uuid)).get_result(connection)
}

// Enable if needed:
// fn find_user_by_email(user_email: &str, connection: &PgConnection) -> QueryResult<SavedUser> {
//     users.filter(email.eq(user_email)).get_result(connection)
// }