use rocket;

use super::service;

pub fn build() -> rocket::Rocket {
    rocket::ignite().mount("/", routes![service::index, service::lessons,])
}
