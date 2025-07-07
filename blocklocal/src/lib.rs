#![allow(async_fn_in_trait)]


use diesel::{Connection, PgConnection};

use crate::environment::Environment;

mod environment;

mod models;
mod schema;

pub mod managers;
pub mod services;
pub mod strategies;
pub mod utils;

pub mod blockchain;
pub mod errors;

/// Establishes a connection to the PostgreSQL database using the DATABASE_URL environment variable.
/// Returns a `PgConnection` if successful, or panics if the connection fails.
fn establish_connection() -> PgConnection {
    let database_url = Environment::new().get_database_url();
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}
