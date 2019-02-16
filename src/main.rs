#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

mod lessons;
mod types;

use lessons as lesson_01;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

fn main() {
    println!("Lesson: {:?}", lesson_01::get_content());
    rocket::ignite().mount("/", routes![index]).launch();
}