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

#[openapi]
#[post("/v1/auth/register", format = "application/json", data = "<request>")]
pub fn auth_register(
    request: Json<RegisterUserRequest>,
    conn: Connection,
) -> Result<Accepted<Json<SuccessRessource>>, ServerError<String>> {
    use crate::schema::users::dsl::*;

    //TODO: taken username

    let new_uuid = Uuid::new_v4();
    let new_user = NewUser {
        uuid: &new_uuid.as_bytes().to_vec(),
        created_at: &chrono::Local::now().naive_utc(),
        updated_at: None,
        username: &request.username,
        first_name: &request.first_name,
        last_name: &request.last_name,
        account_status: &AccountStatus::CREATED,
        timezone: None,
        first_log_in_at: None,
        last_log_in_at: None,
        language: None,
    };

    match insert_into(users).values(&new_user).execute(&*conn) {
        Ok(_) => Ok(Accepted::<Json<SuccessRessource>>(Some(Json(
            SuccessRessource { success: true },
        )))),
        Err(_) => Err(ServerError("Unable to create the user".to_string())),
    }
}
