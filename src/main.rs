#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate serde_derive;

use crate::dotenv::dotenv;
use std::env;

mod db;
mod models;
mod ressources;
mod routes;
mod schema;

#[launch]
fn rocket() -> _ {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    rocket::build()
        .manage(db::establish_connection(database_url))
        .mount("/", routes![routes::api::index::index])
}
