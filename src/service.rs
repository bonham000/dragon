use rocket_contrib::json::Json;

use super::lessons;
use super::lessons::types::LessonSet;

#[derive(Serialize, Deserialize, Debug)]
pub struct LessonScore {
    pub mc: bool,
    pub q: bool,
}

pub type ScoreHistory = Vec<LessonScore>;

#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub uuid: String,
    pub email: String,
    pub experience_points: u64,
    pub score_history: ScoreHistory,
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
pub fn find_or_create_user(user: Json<User>) -> &'static str {
    println!("User: {:?}", user);
    "OK"
}

#[post("/set-score-status/<user_id>", format = "json", data = "<status>")]
pub fn set_score_status(user_id: u64, status: Json<ScoreHistory>) -> &'static str {
    // TODO
    println!("Setting score status for user: {:?}", user_id);
    "OK"
}
