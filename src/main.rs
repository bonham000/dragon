#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate diesel;

extern crate dotenv;
use dotenv::dotenv;

use rocket_contrib::json::Json;

mod lessons;

#[get("/rocket")]
fn index() -> &'static str {
    "Hello, world!"
}

#[get("/lessons")]
fn lessons() -> Json<Vec<lessons::types::Lesson>> {
    let content = lessons::aggregate::get_content();
    println!("Lesson: {:?}", content);

    Json(content)
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index, lessons])
        .launch();
}
