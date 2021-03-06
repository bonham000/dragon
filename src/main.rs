#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate diesel;

extern crate dotenv;
use dotenv::dotenv;

mod db;
mod repository;
mod routes;
mod schema;
mod service;
mod types;
mod utils;

/// Program entry point
fn main() {
    // Load environment variables
    dotenv().ok();

    // Build the routes
    let dragon = routes::build();

    // Launch the server
    dragon.launch();
}
