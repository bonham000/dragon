use diesel;
use diesel::prelude::*;

use super::schema::users::dsl::*;
use super::types::{InitialUserData, InsertableUser, SavedUser};
use super::utils::create_new_user;

pub fn get_user(user_uuid: String, connection: &PgConnection) -> Result<SavedUser, String> {
    println!("Fetching existing user");
    let result = find_user_by_uuid(&user_uuid, connection);
    match result {
        Ok(user) => Ok(user),
        Err(_) => Err("Error fetching user!".to_string()),
    }
}

fn find_user_by_uuid(user_uuid: &str, connection: &PgConnection) -> QueryResult<SavedUser> {
    users.filter(uuid.eq(user_uuid)).get_result(connection)
}

pub fn create_user(user: InitialUserData, connection: &PgConnection) -> Result<SavedUser, String> {
    println!("Creating new user");
    let user = create_new_user(user);
    let result = insert_new_user(user, connection);
    match result {
        Ok(user) => Ok(user),
        Err(_) => Err("Something bad happened!".to_string()),
    }
}

fn insert_new_user(user: InsertableUser, connection: &PgConnection) -> QueryResult<SavedUser> {
    diesel::insert_into(users)
        .values(&user)
        .get_result(connection)
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
