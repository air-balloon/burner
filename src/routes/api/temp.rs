use diesel::dsl::insert_into;
use diesel::{self, prelude::*};
use rocket::response::status::Accepted;
use rocket::serde::json::Json;
use uuid::Uuid;

use crate::db::Connection;
use crate::models::enums::AccountStatus;
use crate::models::user::{NewUser, User};
use crate::requests::register_user::RegisterUserRequest;
use crate::ressources::errors::server_error::ServerError;
use crate::ressources::success::SuccessRessource;
use crate::ressources::user_ressource::UserRessource;

#[openapi]
#[get("/users")]
pub fn get_users(conn: Connection) -> Result<Json<Vec<UserRessource>>, ServerError<String>> {
    use crate::schema::users::dsl::*;
    let mut to_send: Vec<UserRessource> = Vec::new();

    let users_found = users.load::<User>(&*conn).unwrap();
    let default_uuid: Uuid = Uuid::parse_str("000000000000000000000000000000000000").unwrap();
    for user in users_found {
        let _uuid = match Uuid::from_slice(user.uuid.as_slice()) {
            Ok(_uuid) => _uuid,
            Err(_err) => default_uuid,
        };
        to_send.push(UserRessource {
            uuid: _uuid.to_string(),
            username: user.username,
        })
    }
    Ok(Json(to_send))
}
