use diesel::{self, prelude::*};
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket::State;
use shiplift::Docker;
use uuid::Uuid;

use crate::db::Connection;
use crate::models::user::User;
use crate::ressources::info::VersionRessource;
use crate::ressources::user_ressource::UserRessource;

#[openapi]
#[get("/")]
pub fn index(conn: Connection) -> Result<Json<Vec<UserRessource>>, String> {
    use crate::schema::users::dsl::*;
    let mut to_send: Vec<UserRessource> = Vec::new();

    let users_found = users.load::<User>(&*conn).unwrap();

    for user in users_found {
        let _uuid = match Uuid::from_slice(user.uuid.as_slice()) {
            Ok(_uuid) => _uuid,
            Err(_err) => Uuid::parse_str("000000000000000000000000000000000000").unwrap(),
        };
        to_send.push(UserRessource {
            uuid: _uuid.to_string(),
            username: user.username,
        })
    }
    Ok(Json(to_send))
}

#[openapi]
#[get("/docker/version")]
pub async fn docker_version(docker: &State<Docker>) -> Result<Json<VersionRessource>, String> {
    match docker.version().await {
        Ok(ver) => Ok(Json(VersionRessource {
            version: ver.version,
            api_version: ver.api_version,
            kernel_version: ver.kernel_version,
        })),
        Err(_) => Err(Status::ServiceUnavailable.to_string()),
    }
}
