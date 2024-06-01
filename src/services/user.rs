use actix_web::web::Json;
use diesel::prelude::*;
use diesel::result::Error as DieselError;
use uuid::Uuid;

use crate::db::schema::users::dsl::*;
use crate::messages::user::*;
use crate::models::user::User;

pub async fn read_by_id(conn: &mut PgConnection, id: Uuid) -> Result<User, DieselError> {
    return users.find(id).select(User::as_select()).first(conn);
}

pub async fn read_by_username(conn: &mut PgConnection, username: String) {
    return users.filter(User.username.eq(username));
}

pub async fn read_by_email(conn: &mut PgConnection, email: String) {}

pub async fn update(conn: &mut PgConnection, id: String, body: Json<UpdateBodyMessage>) {}
