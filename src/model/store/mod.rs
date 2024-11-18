// region:    --- Modules

mod error;

pub use self::error::{Error, Result};
use std::time::Duration;

use crate::config;
use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};

// endregion: --- Modules

pub type Db = Pool<Postgres>;

pub async fn new_db_pool() -> Result<Db> {
    PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_millis(500))
        .connect(&config().DB_URL)
        .await
        .map_err(|ex| Error::FailToCreatePool(ex.to_string()))
}