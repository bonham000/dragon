#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

mod lessons;

#[get("/rocket")]
fn index() -> &'static str {
    "Hello, world!"
}

fn main() {
    println!("Lesson: {:?}", lessons::lesson_01::get_content());
    rocket::ignite().mount("/", routes![index]).launch();
}