use rocket::http::Status;
use rocket::Response;
use rocket_contrib::json::Json;
use serde_json;

use super::db::DbConn;
use super::repository;

use super::types::{MaybeUser, SavedUser};

#[get("/rocket")]
pub fn index() -> &'static str {
    "Hello from Rocket! 🚀"
}

#[post("/users", format = "json", data = "<user>")]
pub fn find_or_create_user(
    user: Json<MaybeUser>,
    db: DbConn,
) -> Result<Json<SavedUser>, Response<'static>> {
    let user_data = user.into_inner();
    let result = repository::find_or_create_user(user_data, &db);

    match result {
        Ok(user) => Ok(Json(user)),
        Err(_) => Err(get_failure_status()),
    }
}

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

#[delete("/users/<user_id>")]
pub fn remove_user(user_id: String, db: DbConn) -> Result<String, Response<'static>> {
    let result = repository::delete_user(user_id, &db);

    match result {
        Ok(_) => Ok(String::from("OK")),
        Err(e) => {
            println!("Error deleting user: {:?}", e);
            Err(get_failure_status())
        }
    }
}

fn get_failure_status() -> Response<'static> {
    Response::build()
        .status(Status::InternalServerError)
        .finalize()
}
