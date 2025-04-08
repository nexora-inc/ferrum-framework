use std::{env, time::Duration};

use sqlx::{postgres::PgPoolOptions, PgPool};

use crate::Error;

pub async fn init_pool() -> Result<PgPool, Error> {
  let database_url = env::var("DB_URL")
    .unwrap_or("".to_string());

  Ok(PgPoolOptions::new()
    .max_connections(5)
    .acquire_timeout(Duration::from_secs(5))
    .idle_timeout(Duration::from_secs(60 * 5))
    .max_lifetime(Some(Duration::from_secs(60 * 30)))
    .connect(&database_url)
    .await?)
}
