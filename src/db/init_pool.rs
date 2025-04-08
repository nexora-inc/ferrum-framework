use std::env;

use sqlx::PgPool;

use crate::Error;

pub async fn init_pool() -> Result<PgPool, Error> {
  let database_url = env::var("DB_URL")
    .unwrap_or("".to_string());

  Ok(PgPool::connect(&database_url).await?)
}
