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
                service::get_user,
                service::create_user,
                service::update_user,
                service::remove_user,
            ],
        )
        .mount("/static", StaticFiles::from("static"))
}
