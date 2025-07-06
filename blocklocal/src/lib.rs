#![allow(async_fn_in_trait)]

mod schema;

pub mod strategies;
pub mod utils;

pub mod blockchain;
pub mod errors;

fn establish_connection() -> PgConnection {
    let database_url = dotenv::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}
