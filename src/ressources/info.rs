use schemars::JsonSchema;
use serde::Serialize;

#[derive(Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct VersionRessource {
    pub version: String,
    pub api_version: String,
    pub kernel_version: String,
}

#[derive(Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct ApiRessource {
    pub version: String,
    pub message: String,
}
