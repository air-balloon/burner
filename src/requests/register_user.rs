use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct RegisterUserRequest {
    #[schemars(example = "example_username")]
    pub username: String,
    #[schemars(example = "example_first_name")]
    pub first_name: String,
    #[schemars(example = "example_last_name")]
    pub last_name: String,
    #[schemars(example = "example_password")]
    pub password: String,
}

pub fn example_username() -> &'static str {
    "groot"
}

pub fn example_first_name() -> &'static str {
    "Raven"
}

pub fn example_last_name() -> &'static str {
    "Rayes"
}

pub fn example_password() -> &'static str {
    "MyPassw0rd"
}
