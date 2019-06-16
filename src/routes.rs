use rocket;

use super::db;
use super::service;

/// Build the routes for the Rocket server and initialize
/// the database pool connection
pub fn build() -> rocket::Rocket {
    rocket::ignite().manage(db::init_pool()).mount(
        "/",
        routes![
            service::index,
            service::get_user,
            service::create_user,
            service::update_user,
            service::delete_user,
        ],
    )
}
