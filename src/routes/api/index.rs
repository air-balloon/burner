use rocket::serde::json::Json;
use rocket::State;
use shiplift::Docker;

use crate::ressources::errors::server_error::ServerError;
use crate::ressources::info::{ApiRessource, VersionRessource};

#[openapi]
#[get("/")]
pub fn index() -> Result<Json<ApiRessource>, ServerError<String>> {
    Ok(Json(ApiRessource {
        version: "".to_string(),
        message: "Let the air heated up by the burner warm your could.".to_string(),
    }))
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
