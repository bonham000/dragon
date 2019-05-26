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
                service::find_or_create_user,
                service::update_user,
                service::set_scores,
                service::set_experience_points,
                service::set_app_difficulty_setting,
                service::remove_user,
            ],
        )
        .mount("/static", StaticFiles::from("static"))
}
