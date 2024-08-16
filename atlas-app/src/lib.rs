mod services;
mod utils;
mod models;
mod schema;
mod tests;

use diesel::r2d2::{self, ConnectionManager};
use diesel::sqlite::SqliteConnection;
use std::env;

type Pool = r2d2::Pool<ConnectionManager<SqliteConnection>>;
type DbConn = r2d2::PooledConnection<ConnectionManager<SqliteConnection>>;

pub fn establish_connection_pool() -> Pool {
    dotenvy::dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<SqliteConnection>::new(database_url);
    Pool::builder().build(manager).expect("Failed to create pool.")
}
