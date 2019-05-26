use rocket::http::Status;
use rocket::Response;
use rocket_contrib::json::Json;
use serde_json;

use super::db::DbConn;
use super::repository;

use super::types::{ScoreHistory, ExpJson, DifficultySetting, MaybeUser, SavedUser};

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

#[post("/users-update", format = "json", data = "<user>")]
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

#[post("/set-scores/<user_id>", format = "json", data = "<scores_json>")]
pub fn set_scores(
    user_id: String,
    scores_json: Json<ScoreHistory>,
    db: DbConn,
) -> Result<Json<SavedUser>, Response<'static>> {
    println!("Setting score status for user: {:?}", user_id);
    // TODO:
    // - Authenticate request again Google APIs with provided request header access token

    let scores = scores_json.into_inner();
    let scores_string = serde_json::to_string(&scores);

    match scores_string {
        Ok(scores) => {
            let result = repository::update_user_scores(user_id, scores, &db);

            match result {
                Ok(user) => Ok(Json(user)),
                Err(e) => {
                    println!("Could not update user scores: {:?}", e);
                    Err(get_failure_status())
                }
            }
        }
        Err(e) => {
            println!("Error decoding user scores: {:?}", e);
            Err(get_failure_status())
        }
    }
}

#[post("/experience/<user_id>", format = "json", data = "<exp>")]
pub fn set_experience_points(
    user_id: String,
    exp: Json<ExpJson>,
    db: DbConn,
) -> Result<Json<SavedUser>, Response<'static>> {
    println!("Updating experience for user: {:?}", user_id);
    // TODO
    // - Authenticate request again Google APIs with provided request header access token

    let experience_points = exp.into_inner().experience_points;
    let exp = experience_points.parse::<i64>();

    match exp {
        Ok(exp) => {
            let result = repository::set_experience_points(user_id, exp, &db);

            match result {
                Ok(user) => Ok(Json(user)),
                Err(e) => {
                    println!("Error setting user experience points: {:?}", e);
                    Err(get_failure_status())
                }
            }
        }
        Err(e) => {
            println!("Error decoding user experience points: {:?}", e);
            Err(get_failure_status())
        }
    }
}

#[post(
    "/difficulty/<user_id>",
    format = "json",
    data = "<app_difficulty_setting>"
)]
pub fn set_app_difficulty_setting(
    user_id: String,
    app_difficulty_setting: Json<DifficultySetting>,
    db: DbConn,
) -> Result<Json<SavedUser>, Response<'static>> {
    // TODO
    // - Authenticate request again Google APIs with provided request header access token

    let setting_json = app_difficulty_setting.into_inner();
    let setting = setting_json.app_difficulty_setting;

    println!(
        "Updating app difficulty setting for user: {:?} to {:?}",
        user_id, setting
    );
    let result = repository::set_app_difficulty_setting(user_id, setting, &db);

    match result {
        Ok(user) => Ok(Json(user)),
        Err(e) => {
            println!("Error setting user experience points: {:?}", e);
            Err(get_failure_status())
        }
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
