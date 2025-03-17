//! This module contains the database connection and the models for the blocklocal database.

pub mod errors;
pub mod networks;
pub mod profiles;

pub use errors::DbResult;
use tokio::runtime::Runtime;

use std::{sync::LazyLock, time::Duration};

use sea_orm::{ConnectOptions, Database, DatabaseConnection};

static DB: LazyLock<DatabaseConnection> = LazyLock::new(|| {
    let rt = Runtime::new().expect("Failed to create tokio runtime");
    let url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let mut opt = ConnectOptions::new(url);
    opt.max_connections(100)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .acquire_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8))
        .sqlx_logging(true)
        .sqlx_logging_level(log::LevelFilter::Info)
        .set_schema_search_path("my_schema");
    let connection = rt
        .block_on(Database::connect(opt))
        .expect("Failed to connect to database");

    connection
});
