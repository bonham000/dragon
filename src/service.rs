use diesel;
use rocket_contrib::json::Json;
use serde::ser::{Serialize, SerializeStruct, Serializer};
use serde_json;
use uuid::Uuid;

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
pub struct NewUser {
    pub email: String,
}

#[derive(Queryable, Insertable, Serialize, Deserialize, Debug)]
#[table_name = "users"]
pub struct InsertableUser {
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
pub fn find_or_create_user(user: Json<NewUser>, db: DbConn) -> &'static str {
    let user_data = user.into_inner();
    // If user does not exist, create
    let new_user = InsertableUser {
        email: user_data.email,
        uuid: Uuid::new_v4().to_string(),
        experience_points: 0,
        score_history: "".to_string(),
    };

    repository::find_or_create_user(new_user, &db);
    "OK"
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
