use sqlx::PgPool;

use crate::Error;

use super::{init_pool, DB_POOL};

pub async fn get_pool() -> Result<&'static PgPool, Error> {
  Ok(DB_POOL.get_or_try_init(init_pool).await?)
}
