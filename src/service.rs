use diesel;
use rocket::http::Status;
use rocket::Response;
use rocket_contrib::json::Json;
use serde_json;

use super::schema::users;

use super::db::DbConn;
use super::lessons;
use super::lessons::types::LessonSet;
use super::repository;

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct LessonScore {
    pub mc: bool,
    pub q: bool,
}

pub type ScoreHistory = Vec<LessonScore>;

#[derive(Serialize, Deserialize, Debug)]
pub struct MaybeUser {
    pub email: String,
}

#[derive(Queryable, Insertable, Serialize, Deserialize, Debug, PartialEq, Eq)]
#[table_name = "users"]
pub struct InsertableUser {
    pub email: String,
    pub uuid: String,
    pub experience_points: i64,
    pub score_history: String,
}

#[derive(Queryable, AsChangeset, Serialize, Deserialize, Debug, Identifiable, PartialEq, Eq)]
#[table_name = "users"]
pub struct SavedUser {
    pub id: i32,
    pub email: String,
    pub uuid: String,
    pub experience_points: i64,
    pub score_history: String,
}

#[get("/rocket")]
pub fn index() -> &'static str {
    "Hello from Rocket! 🚀"
}

#[get("/lessons")]
pub fn lessons() -> Json<LessonSet> {
    let lesson_set = lessons::aggregate::get_content();
    Json(lesson_set)
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

#[post("/set-scores/<user_id>", format = "json", data = "<scores_json>")]
pub fn set_scores(user_id: String, scores_json: Json<ScoreHistory>) -> &'static str {
    // TODO
    // - Authenticate request again Google APIs with provided request header access token
    // - Save provided score status against user in database
    println!("Setting score status for user: {:?}", user_id);

    let scores = scores_json.into_inner();
    let scores_string = serde_json::to_string(&scores);
    println!("Scores: {:?}", scores_string);
    "OK"
}

fn get_failure_status() -> Response<'static> {
    Response::build()
        .status(Status::InternalServerError)
        .finalize()
}
