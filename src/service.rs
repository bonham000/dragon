use rocket::Response;
use rocket_contrib::json::Json;

use super::db::DbConn;
use super::repository;

use super::types::{InitialUserData, SavedUser};
use super::utils::get_failure_status;

/// Placeholder GET route
#[get("/rocket")]
pub fn index() -> &'static str {
    "Hello from Rocket! 🚀"
}

/// Handler to get an existing user, if they exist
///
/// * `user_uuid` - user uuid
/// * `db` - database connection
#[get("/users/<user_uuid>")]
pub fn get_user(user_uuid: String, db: DbConn) -> Result<Json<SavedUser>, Response<'static>> {
    let result = repository::get_user(user_uuid, &db);

    match result {
        Ok(user) => Ok(Json(user)),
        Err(_) => Err(get_failure_status()),
    }
}

/// Handler to create a new user
///
/// * `user` - initial user data to be used in user creation
/// * `db` - database connection
#[post("/users", format = "json", data = "<user>")]
pub fn create_user(
    user: Json<InitialUserData>,
    db: DbConn,
) -> Result<Json<SavedUser>, Response<'static>> {
    let user_data = user.into_inner();
    let result = repository::create_user(user_data, &db);

    match result {
        Ok(user) => Ok(Json(user)),
        Err(_) => Err(get_failure_status()),
    }
}

/// Handler to update an existing user
///
/// * `user` - user data to save
/// * `db` - database connection
#[put("/users", format = "json", data = "<user>")]
pub fn update_user(
    user: Json<SavedUser>,
    db: DbConn,
) -> Result<Json<SavedUser>, Response<'static>> {
    let user_data = user.into_inner();
    let result = repository::update_user(user_data, &db);

    match result {
        Ok(user) => Ok(Json(user)),
        Err(_) => Err(get_failure_status()),
    }
}

/// Handler to delete an existing user
///
/// * `user_uuid` - user uuid
/// * `db` - database connection
#[delete("/users/<user_uuid>")]
pub fn delete_user(user_uuid: String, db: DbConn) -> Result<String, Response<'static>> {
    let result = repository::delete_user(user_uuid, &db);

    match result {
        Ok(_) => Ok(String::from("OK")),
        Err(e) => {
            println!("Error deleting user: {:?}", e);
            Err(get_failure_status())
        }
    }
}
