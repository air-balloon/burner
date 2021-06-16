use schemars::JsonSchema;
use serde::Serialize;

#[derive(Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct UserRessource {
    pub uuid: String,
    pub username: String,
}
