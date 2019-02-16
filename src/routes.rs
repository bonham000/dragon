use rocket;

use super::service;

pub fn build() -> rocket::Rocket {
    rocket::ignite().mount(
        "/",
        routes![
            service::index,
            service::lessons,
            service::find_or_create_user,
            service::set_score_status
        ],
    )
}
