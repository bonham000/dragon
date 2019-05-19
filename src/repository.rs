use ::uuid::Uuid;
use diesel;
use diesel::prelude::*;
use serde_json;

use super::schema::users::dsl::*;
use super::service::{InsertableUser, MaybeUser, SavedUser, ScoreHistory};

pub fn find_or_create_user(
    user: MaybeUser,
    connection: &PgConnection,
) -> Result<SavedUser, String> {
    let user_email = user.email;
    let maybe_user: QueryResult<SavedUser> = find_user_by_email(&user_email, connection);

    match maybe_user {
        Ok(user) => {
            println!("User already exists.");
            Ok(user)
        }
        Err(diesel::result::Error::NotFound) => {
            println!("User doesn't exist, creating new user.");
            let user = create_new_user(user_email);
            let result = insert_new_user(user, connection);
            match result {
                Ok(user) => Ok(user),
                Err(_) => Err("Something bad happened!".to_string()),
            }
        }
        Err(e) => {
            println!("Error in find_or_create_user... {:?}", e);
            Err("Something bad happened!".to_string())
        }
    }
}

pub fn update_user_scores(
    user_uuid: String,
    scores: String,
    connection: &PgConnection,
) -> QueryResult<SavedUser> {
    diesel::update(users.filter(uuid.eq(user_uuid)))
        .set(score_history.eq(scores))
        .get_result(connection)
}

pub fn set_experience_points(
    user_uuid: String,
    exp: i64,
    connection: &PgConnection,
) -> QueryResult<SavedUser> {
    diesel::update(users.filter(uuid.eq(user_uuid)))
        .set(experience_points.eq(exp))
        .get_result(connection)
}

pub fn set_app_difficulty_setting(
    user_uuid: String,
    setting: String,
    connection: &PgConnection,
) -> QueryResult<SavedUser> {
    diesel::update(users.filter(uuid.eq(user_uuid)))
        .set(app_difficulty_setting.eq(setting))
        .get_result(connection)
}

pub fn delete_user(user_uuid: String, connection: &PgConnection) -> QueryResult<usize> {
    diesel::delete(users.filter(uuid.eq(user_uuid))).execute(connection)
}

fn find_user_by_email(user_email: &str, connection: &PgConnection) -> QueryResult<SavedUser> {
    users.filter(email.eq(user_email)).get_result(connection)
}

fn insert_new_user(user: InsertableUser, connection: &PgConnection) -> QueryResult<SavedUser> {
    diesel::insert_into(users)
        .values(&user)
        .get_result(connection)
}

fn create_new_user(user_email: String) -> InsertableUser {
    let default_score_history = ScoreHistory {
        mc_english: false,
        mc_mandarin: false,
        quiz_text: false,
        mandarin_pronunciation: false,
        final_completed_lesson_index: 0,
    };

    InsertableUser {
        email: user_email,
        uuid: Uuid::new_v4().to_string(),
        experience_points: 0,
        score_history: serde_json::to_string(&default_score_history).unwrap(),
        app_difficulty_setting: "MEDIUM".to_string(),
    }
}
