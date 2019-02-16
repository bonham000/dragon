use rocket_contrib::json::Json;

use super::lessons;
use super::lessons::types::LessonSet;

#[get("/rocket")]
pub fn index() -> &'static str {
    "Hello, world!"
}

#[get("/lessons")]
pub fn lessons() -> Json<LessonSet> {
    let lesson_set = lessons::aggregate::get_content();
    Json(lesson_set)
}
