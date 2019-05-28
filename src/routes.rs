use rocket;
use rocket_contrib::serve::StaticFiles;

use super::db;
use super::service;

pub fn build() -> rocket::Rocket {
    rocket::ignite()
        .manage(db::init_pool())
        .mount(
            "/",
            routes![
                service::index,
                service::update_user,
                service::remove_user,
                service::find_or_create_user,
            ],
        )
        .mount("/static", StaticFiles::from("static"))
}
