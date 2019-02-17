use diesel;
use diesel::prelude::*;

use super::schema::users::dsl::*;
use super::service::InsertableUser;

pub fn find_or_create_user(user: InsertableUser, connection: &PgConnection) -> InsertableUser {
    println!("User: {:?}", user);

    user
}
