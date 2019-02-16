#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate diesel;

extern crate dotenv;
use dotenv::dotenv;

mod lessons;
mod routes;
mod service;

fn main() {
    let dragon = routes::build();

    dragon.launch();
}
