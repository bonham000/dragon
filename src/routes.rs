use rocket;

use super::db;
use super::service;

pub fn build() -> rocket::Rocket {
    rocket::ignite().manage(db::init_pool()).mount(
        "/",
        routes![
            service::index,
            service::lessons,
            service::find_or_create_user,
            service::set_score_status
        ],
    )
}