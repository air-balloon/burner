use diesel::{self, prelude::*};
use rocket::serde::json::Json;
use rocket::State;
use shiplift::Docker;
use uuid::Uuid;

use crate::db::Connection;
use crate::models::user::User;
use crate::ressources::errors::server_error::ServerError;
use crate::ressources::info::{ApiRessource, VersionRessource};
use crate::ressources::user_ressource::UserRessource;

#[openapi]
#[get("/")]
pub fn index() -> Result<Json<ApiRessource>, ServerError<String>> {
    Ok(Json(ApiRessource {
        version: "".to_string(),
        message: "Let the air heated up by the burner warm your could.".to_string(),
    }))
}

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

#[openapi]
#[get("/docker/version")]
pub async fn docker_version(
    docker: &State<Docker>,
) -> Result<Json<VersionRessource>, ServerError<String>> {
    match docker.version().await {
        Ok(ver) => Ok(Json(VersionRessource {
            version: ver.version,
            api_version: ver.api_version,
            kernel_version: ver.kernel_version,
        })),
        Err(_) => Err(ServerError(
            "Unable to fetch the docker version".to_string(),
        )),
    }
}
