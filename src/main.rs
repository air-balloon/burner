#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;
extern crate dotenv;
#[macro_use]
extern crate rocket_okapi;
extern crate serde;
extern crate serde_derive;
extern crate serde_json;

use crate::dotenv::dotenv;
use rocket_okapi::routes_with_openapi;
use rocket_okapi::swagger_ui::{make_swagger_ui, SwaggerUIConfig};
use std::env;

use shiplift::Docker;

mod db;
mod models;
mod ressources;
mod routes;
mod schema;

#[launch]
fn rocket() -> _ {
    println!("Let the air heated up by the burner warm your could.");
    dotenv().ok();

    let docker_conn = Docker::new();
    //.expect("Impossible to connect to Docker");

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    rocket::build()
        .manage(db::establish_connection(database_url))
        .manage(docker_conn)
        .mount(
            "/dev/swagger-ui/",
            make_swagger_ui(&SwaggerUIConfig {
                url: "../../openapi.json".to_owned(),
                ..Default::default()
            }),
        )
        .mount(
            "/",
            routes_with_openapi![
                routes::api::index::index,
                routes::api::index::docker_version
            ],
        )
}
