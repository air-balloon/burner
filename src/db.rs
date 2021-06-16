use rocket::http::Status;
use rocket::outcome::{try_outcome, Outcome};
use rocket::request::{self, FromRequest};
use rocket::{Request, State};
use std::ops::Deref;

use diesel::r2d2;
use diesel::r2d2::ConnectionManager;

use diesel::mysql::MysqlConnection;

pub type Pool = r2d2::Pool<ConnectionManager<MysqlConnection>>;

pub fn establish_connection(database_url: String) -> Pool {
    let manager = ConnectionManager::<MysqlConnection>::new(database_url);
    r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool")
}

pub type PooledConnection = r2d2::PooledConnection<ConnectionManager<MysqlConnection>>;

// Connection request guard type: a wrapper around an r2d2 pooled connection.
pub struct Connection(pub PooledConnection);

/// Attempts to retrieve a single connection from the managed database pool. If
/// no pool is currently managed, fails with an `InternalServerError` status. If
/// no connections are available, fails with a `ServiceUnavailable` status.
#[rocket::async_trait]
impl<'r> FromRequest<'r> for Connection {
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let pool = try_outcome!(req.guard::<&State<Pool>>().await);
        match pool.get() {
            Ok(conn) => Outcome::Success(Connection(conn)),
            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ())),
        }
    }
}

// For the convenience of using an &Connection as an &MysqlConnection.
impl Deref for Connection {
    type Target = MysqlConnection;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
